use axum::{serve::Serve, Router};
use repositories::Database;
use tokio::signal;
use tracing::{event, Level};

fn before_axum() {
    event!(Level::WARN, "The server is shutting down!");
    event!(Level::INFO, "Waiting for pending requests (max. 15s)...");
}

async fn after_axum() {
    event!(Level::DEBUG, "All pending requests have been processed!");
    Database::disconnect().await;
}

pub async fn with_graceful_shutdown(axum_serve: Serve<Router, Router>) {
    drop(
        axum_serve
            .with_graceful_shutdown(async {
                shutdown_signal().await;
                before_axum();
            })
            .await,
    );
    after_axum().await;
}

/// # Panics
///
/// Will panic if fails to install any of the signal handlers.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap_or_else(|e| {
            panic!("Failed to install Ctrl+C handler! {e}")
        });
    };

    #[cfg(unix)]
    let term_or_int = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap_or_else(|e| {
                panic!("Failed to install SIGTERM handler! {e}")
            })
            .recv()
            .await;
    };

    #[cfg(windows)]
    let term_or_int = async {
        signal::windows::ctrl_close()
            .unwrap_or_else(|e| {
                panic!("Failed to install Windows SIGINT handler! {e}")
            })
            .recv()
            .await;
    };

    tokio::select! {
      () = ctrl_c => {},
      () = term_or_int => {},
    }
}
