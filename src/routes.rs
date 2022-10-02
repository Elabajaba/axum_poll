// API:
// `GET /polls` to get a JSON list of polls.//
// `POST /polls` to create a new poll.
// `GET /polls/:id` to get a specific poll that you can vote on.//
// `PUT /polls/:id` to update a poll's options.
// `POST /polls/:id` for submitting poll selections.
// TODO: Server Send Events or Websockets for sending poll updates to clients?

use std::str::FromStr;

use crate::poll::{CreatePoll, DatabasePoll};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rusty_ulid::{generate_ulid_string, Ulid};
use sqlx::{Acquire, SqlitePool};

// TODO: Separate database calls from routes, and have database calls return errors.

pub(crate) async fn get_all_polls(Extension(db): Extension<SqlitePool>) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = db.acquire().await.unwrap(); // TODO: Error handling middleware.

    let polls = sqlx::query!(
        r#"
        SELECT poll_id as poll_id, title as title
        FROM polls
        ORDER BY poll_id
        "#
    )
    .fetch_all(&mut connection)
    .await
    .unwrap();

    let polls: Vec<DatabasePoll> = polls
        .iter()
        .map(|rec| {
            let poll_id_string = String::from_utf8(rec.poll_id.clone()).unwrap();
            let poll_id = Ulid::from_str(&poll_id_string).unwrap();
            DatabasePoll {
                poll_id,
                title: rec.title.to_owned(),
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(polls)))
}

// Creates a new poll
// Title: Self explanatory
// Options: The options you can vote for in the poll (todo: min+max option limits? max option length?)
// Multi: bool; does the poll accept multiple votes, or is it a single vote poll
// Potential future options: captcha and duplicate vote checking.
// returns a Poll
pub(crate) async fn post_new_poll(
    Json(input): Json<CreatePoll>,
    Extension(db): Extension<SqlitePool>,
) -> impl IntoResponse {
    // TODO: See if we can directly insert Ulids as bytes into the database, instead of converting to strings.
    let id = generate_ulid_string();

    let mut connection = db.acquire().await.unwrap(); // TODO: This is fallible.

    let mut transaction = connection.begin().await.unwrap(); // TODO: This is fallible.

    // Create the poll.
    let _created_poll = sqlx::query_as!(
        Poll,
        r#"
        INSERT INTO polls ( poll_id, title, multi )
        VALUES ( ?1, ?2, ?3 )
        "#,
        id,
        input.title,
        input.multi,
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    // TODO: Batch this somehow? Currently doing way too many transactions here.
    for option in input.options {
        // TODO: See if we can directly insert Ulids as bytes into the database, instead of converting to strings.
        let option_id = generate_ulid_string();

        sqlx::query!(
            r#"
        INSERT INTO poll_options ( poll_option_id, poll_id, option )
        VALUES ( ?1, ?2, ?3 )
        "#,
            option_id,
            id,
            option
        )
        .execute(&mut transaction)
        .await
        .unwrap();
    }

    transaction.commit().await.unwrap();

    (StatusCode::CREATED, Json(id))

    // poll_option_id          BLOB PRIMARY KEY    NOT NULL,
    // poll_id     BLOB                NOT NULL,
    // option      TEXT                NOT NULL,
    // votes       INT                 NOT NULL    DEFAULT 0,
    // constraint fk_polls foreign key(poll_id) references polls(id)
}

pub(crate) async fn put_update_poll(Path(_id): Path<Ulid>) {
    unimplemented!()
}

pub(crate) async fn get_single_poll(Path(_id): Path<Ulid>) {
    unimplemented!()
}

// TODO: How do I send the vote choices, and how do I extract them.
pub(crate) async fn post_vote_poll(Path(_id): Path<Ulid>) {
    unimplemented!()
}
