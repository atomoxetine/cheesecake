use config::PORT;

pub mod config;
pub mod controller;
pub mod view;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    tracing_subscriber::fmt::init();

    let app = controller::router();

    let host = format!("0.0.0.0:{}", *PORT);
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

