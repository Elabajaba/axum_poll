use std::{error::Error, net::SocketAddr, time::Duration};

use axum::{error_handling::HandleErrorLayer, extract::Extension, routing::get, Router, http::{Method, Uri, StatusCode}, BoxError};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_error::ErrorLayer;
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
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the .env vars.
    // dotenvy::dotenv().ok();
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_poll=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(ErrorLayer::default())
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
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(30)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{} {}` failed with {}", method, uri, err),
    )
}
