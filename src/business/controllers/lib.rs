mod app_rejection;
mod v1;

use axum::Router;

pub trait Routes {
    #[must_use]
    fn configure_routes(self) -> Self;
}
impl Routes for Router {
    fn configure_routes(self) -> Self {
        self.nest("/api", Self::new().nest("/v1", v1::router()))
    }
}
