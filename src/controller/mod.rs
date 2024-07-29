use axum::Router;
use tower_http::services::ServeDir;

mod index;

pub fn router() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/", index::router())
}
