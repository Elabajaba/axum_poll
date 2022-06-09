use std::net::SocketAddr;

use axum::{extract::Extension, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod poll;
mod routes;
mod templates;

// API:
// `GET /polls` to get a JSON list of polls.//
// `POST /polls` to create a new poll.
// `GET /polls/:id` to get a specific poll that you can vote on.//
// `PUT /polls/:id` to update a poll's options.
// `POST /polls/:id` for submitting poll selections.

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Set the RUST_LOG, if it hasn't been explicitly defined
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "axum_poll=debug,tower_http=debug")
    // }
    // tracing_subscriber::fmt::init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_poll=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = db::db().await;

    let polls_api = Router::new()
        .route("/", get(routes::get_all_polls).post(routes::post_new_poll))
        .route(
            "/:id",
            get(routes::get_single_poll)
                .post(routes::post_vote_poll)
                .put(routes::put_update_poll),
        );

    let app = Router::new()
        .nest("/api/polls", polls_api)
        .route("/list/:messages", get(templates::greet))
        .route("/polls", get(templates::temp))
        .layer(Extension(db))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
