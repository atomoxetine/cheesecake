use tokio::signal;
use tracing::{event, Level};

/// # Panics
///
/// Will panic if fails to install any of the signal handlers.
pub async fn shutdown_signal() {
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
      _ = ctrl_c => {},
      _ = term_or_int => {},
    }

    on_before_axum_countdown().await
}

async fn on_before_axum_countdown() {
    event!(
    Level::INFO,
    "The server is shutting down! Waiting for pending requests (max. 20s)..."
  );
}
