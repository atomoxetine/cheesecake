use axum::{routing::get, Router};
use dotenv::dotenv;
use config::PORT;

pub mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let host = format!("0.0.0.0:{}", *PORT);
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello World"
}
