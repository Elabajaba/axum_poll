mod all_polls;
mod single_poll;
mod greet;

pub use all_polls::temp;
pub use greet::greet;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
