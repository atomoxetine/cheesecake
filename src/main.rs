use config::PORT;
use dotenv;

pub mod config;
pub mod utils;
pub mod routing;
pub mod view;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    tracing_subscriber::fmt::init();

    let app = routing::ck_router();

    let host = format!("0.0.0.0:{}", *PORT);
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

