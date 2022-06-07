use askama::Template;
use axum::{response::IntoResponse, extract::Path};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    messages: Vec<String>,  
}

pub async fn greet(Path(messages): Path<Vec<String>>) -> impl IntoResponse {
    let template = HelloTemplate { messages };
    crate::templates::HtmlTemplate(template)
}