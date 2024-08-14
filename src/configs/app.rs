use crate::business::controllers;
use crate::utils::custom_errors::{
    app_error::AppError,
    err_response::{ApiErrResponse, ViewErrResponse},
};
use crate::views::not_found;
use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    Router,
};
use controllers::Routes;
use std::{borrow::Cow, time::Duration};
use tower::{
    limit::ConcurrencyLimitLayer, load_shed::LoadShedLayer, BoxError,
    ServiceBuilder,
};
use tower_http::{
    services::ServeDir, timeout::TimeoutLayer, trace::TraceLayer,
};

pub fn app() -> Router {
    Router::new()
        // Serve static files from the `assets` directory
        .nest_service(
            "/assets",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")),
        )
        .configure_routes()
        .fallback(fallback)
        // Insert here all layers that might fail. Make sure to treat the error in `handle_error`.
        // Axum's philosophy is to ensure layers cannot fail, so when using something like a tower layer that
        // might fail, it is recommended to treat it like this.
        .layer(
            ServiceBuilder::new().layer((
                HandleErrorLayer::new(handle_error),
                LoadShedLayer::new(),
            )),
        )
        .layer(ConcurrencyLimitLayer::new(1024))
        .layer(TimeoutLayer::new(Duration::from_secs(20)))
        .layer(TraceLayer::new_for_http())
}

async fn fallback() -> Result<impl IntoResponse, ViewErrResponse> {
    Ok((StatusCode::NOT_FOUND, Html(not_found::render()?)))
}

async fn handle_error(error: BoxError) -> Response<Body> {
    // If server is overloaded, immediately returns a 503 without further processing the request
    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("Service is overloaded, try again later."),
        )
            .into_response();
    }

    // Handles any other unexpected error just in case
    ApiErrResponse::from(AppError::new_500(anyhow::anyhow!(error)))
        .into_response()
}
