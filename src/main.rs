use std::{
    collections::HashMap,
    convert::Infallible,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use axum::{
    body::{Bytes, Full},
    extract,
    extract::{Extension, Path, Query},
    handler::{get, patch},
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    Json,
    Router,
};
use parking_lot::RwLock;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use tower::{BoxError, ServiceBuilder};
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};
use uuid::Uuid;

mod routes;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
struct HelloTemplate {
    messages: Vec<String>,
}

// API:
// `GET /polls` to get a JSON list of polls.// 
// `POST /polls` to create a new poll.
// `GET /polls/:id` to get a specific poll that you can vote on.// 
// `PUT /polls/:id` to update a poll's options.
// `POST /polls/:id` for submitting poll selections.

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_poll=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
    
    let polls_api = Router::new()
        .route("/", get(routes::get_all_polls).post(routes::post_new_poll))
        .route("/:id", get(routes::get_single_poll).post(routes::post_vote_poll).put(routes::put_update_poll));

    let app = Router::new()
        .nest("/api/polls", polls_api)
        .route("/list/:messages", get(greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(Path(messages): Path<Vec<String>>) -> impl IntoResponse {
    let template = HelloTemplate { messages };
    HtmlTemplate(template)
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: TemplateOnce,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self.0.render_once() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                )))
                .unwrap(),
        }
    }
}
