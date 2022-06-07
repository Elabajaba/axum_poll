use askama::Template;
use axum::{extract::Extension, response::IntoResponse};
use sqlx::SqlitePool;

use crate::poll::{get_database_polls_from_db , DatabasePoll};

#[derive(Template)]
#[template(path = "all-polls.html")]
struct AllPollsTemplate {
    polls: Vec<DatabasePoll>,
}

fn all_polls(polls: Vec<DatabasePoll>) -> AllPollsTemplate {
    AllPollsTemplate { polls }
}

pub async fn temp(Extension(db): Extension<SqlitePool>) -> impl IntoResponse {
    let connection = db.acquire().await.unwrap(); // TODO: Error handling middleware.

    let polls = get_database_polls_from_db(connection).await;

    let template = all_polls(polls);

    super::HtmlTemplate(template)
}

// async fn greet(Path(messages): Path<Vec<String>>) -> impl IntoResponse {
//     let template = HelloTemplate { messages };
//     HtmlTemplate(template)
// }