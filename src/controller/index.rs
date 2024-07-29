use axum::{response::Html, routing::get, Router};

use crate::view::index;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> Html<String> {
    Html(index::render().unwrap())
}

