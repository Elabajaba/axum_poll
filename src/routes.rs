// API:
// `GET /polls` to get a JSON list of polls.//
// `POST /polls` to create a new poll.
// `GET /polls/:id` to get a specific poll that you can vote on.//
// `PUT /polls/:id` to update a poll's options.
// `POST /polls/:id` for submitting poll selections.
// TODO: Server Send Events or Websockets for sending poll updates to clients?

use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json, Router,
};
use uuid::Uuid;

pub(crate) async fn get_all_polls() {
    unimplemented!()
}

pub(crate) async fn post_new_poll() {
    unimplemented!()
}

pub(crate) async fn put_update_poll(Path(id): Path<Uuid>) {
    unimplemented!()
}

pub(crate) async fn get_single_poll(Path(id): Path<Uuid>) {
    unimplemented!()
}

// TODO: How do I send the vote choices, and how do I exract them.
pub(crate) async fn post_vote_poll(Path(id): Path<Uuid>) {
    unimplemented!()
}

