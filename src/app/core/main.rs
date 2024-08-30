use environment::{HOSTNAME, PORT, WORKSPACE_DIR};
#[cfg(not(debug_assertions))]
use repositories::Database;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{event, Level};

mod on_shutdown;
use on_shutdown::with_graceful_shutdown;

mod app;
use app::app;

#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    dotenv::from_filename(WORKSPACE_DIR.join(".env")).ok();
    // Logging - The variables are needed for the lifetime of the program
    let _log_guards = utils::init_logging().await;

    // skip migrations for faster development experience
    #[cfg(not(debug_assertions))]
    {
        // Database auto migration
        event!(Level::INFO, "Running DB migrations...");
        sqlx::migrate!()
            .run(Database::get_pool().await)
            .await
            .unwrap_or_else(|e| panic!("Failed to migrate DB! Error: {e}"));
    }

    #[cfg(debug_assertions)]
    views::setup_hotwatch();

    let sock_addr = SocketAddr::from((*HOSTNAME, *PORT));
    let listener = TcpListener::bind(sock_addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to port! Error: {e}"));

    event!(Level::INFO, "Server running on http://{sock_addr}");
    with_graceful_shutdown(axum::serve(listener, app())).await;
}
