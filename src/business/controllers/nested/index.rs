use axum::response::Html;

use crate::{business::ViewResult, views::nested::index::render};

pub async fn get() -> ViewResult {
    Ok(Html(render()?))
}
