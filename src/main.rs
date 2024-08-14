use business::repositories::DB_CONTEXT;
use configs::environment::{HOSTNAME, PORT};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{event, Level};

use cheesecake::*;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    dotenv::from_filename(concat!(env!("CARGO_MANIFEST_DIR"), "/.env")).ok();

    // Logging - The variables are needed for the lifetime of the program
    let _log_guards = utils::init_logging().await;

    // Database auto migration
    event!(Level::INFO, "Running DB migrations...");
    sqlx::migrate!()
        .run(&DB_CONTEXT.pool)
        .await
        .unwrap_or_else(|e| panic!("Failed to migrate DB! Error: {e}"));

    let sock_addr = SocketAddr::from((*HOSTNAME, *PORT));
    let listener = TcpListener::bind(sock_addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to port! Error: {e}"));

    event!(Level::INFO, "Server running on http://{sock_addr}");
    axum::serve(listener, configs::app())
        .with_graceful_shutdown(configs::shutdown_signal())
        .await
        .unwrap();
}
