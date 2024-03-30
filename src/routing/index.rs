use axum::{response::Html, routing::get, Router};

use crate::{utils::minify_template::MinifyTemplate, view::index::IndexTemplate};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> Html<String> {
    IndexTemplate {}.render_html().unwrap()
}

