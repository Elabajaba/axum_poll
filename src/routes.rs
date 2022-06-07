// API:
// `GET /polls` to get a JSON list of polls.//
// `POST /polls` to create a new poll.
// `GET /polls/:id` to get a specific poll that you can vote on.//
// `PUT /polls/:id` to update a poll's options.
// `POST /polls/:id` for submitting poll selections.
// TODO: Server Send Events or Websockets for sending poll updates to clients?

use crate::poll::{CreatePoll, Poll};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rusty_ulid::{generate_ulid_string, Ulid};
use sqlx::{Executor, SqlitePool, Transaction, Acquire};

pub(crate) async fn get_all_polls() {
    unimplemented!()
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
    let id = generate_ulid_string();

    let mut connection = db.acquire().await.unwrap(); // TODO: This is fallible.
    // We need to hook up error handling middleware to handle not being able to acquire the db.

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

    for option in input.options {
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

pub(crate) async fn put_update_poll(Path(id): Path<Ulid>) {
    unimplemented!()
}

pub(crate) async fn get_single_poll(Path(id): Path<Ulid>) {
    unimplemented!()
}

// TODO: How do I send the vote choices, and how do I extract them.
pub(crate) async fn post_vote_poll(Path(id): Path<Ulid>) {
    unimplemented!()
}
