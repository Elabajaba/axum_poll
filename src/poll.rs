use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, Sqlite};

#[derive(Deserialize, Serialize)]
pub struct Poll {
    pub id: Ulid,
    pub title: String,
    pub multi: bool,
    pub options: Vec<String>,
}

// TODO: poll_id is a String instead of a Ulid because I want to use sqlx's typechecked queries.
#[derive(Deserialize, Serialize)]
pub struct DatabasePoll {
    pub poll_id: Ulid,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct PollOption {
    pub poll_option_id: Ulid,
    pub option: String,
    pub votes: i32,
    pub related_poll_id: Ulid,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePoll {
    pub title: String,
    pub options: Vec<String>,
    pub multi: bool,
}

pub async fn get_database_polls_from_db(mut connection: PoolConnection<Sqlite>) -> Vec<DatabasePoll> {
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
            let poll_id = Ulid::try_from(rec.poll_id.as_slice()).unwrap();
            DatabasePoll {
                poll_id,
                title: rec.title.to_owned(),
            }
        })
        .collect();

        polls
}