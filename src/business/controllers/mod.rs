mod index;
mod nested;

use axum::{routing::get, Router};

pub trait Routes {
    fn configure_routes(self) -> Self;
}
impl Routes for Router {
    fn configure_routes(self) -> Self {
        self.route("/", get(index::get))
            .nest("/nested", nested::router())
    }
}
