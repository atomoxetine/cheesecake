use axum::http::StatusCode;
use axum::response::Html;

use custom_errors::err_response::{res, HtmlResult};
use views::index::render;

pub async fn get() -> HtmlResult {
    res((StatusCode::OK, Html(render()?)))
}
