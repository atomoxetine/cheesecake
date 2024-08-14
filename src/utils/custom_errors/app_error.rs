use std::{error::Error, fmt};

use axum::http::StatusCode;
use tracing::{event, Level};
use uuid::Uuid;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
    pub identifier: Option<Uuid>,
    pub source: Option<anyhow::Error>,
}

impl AppError {
    pub fn new(
        message: String,
        status_code: StatusCode,
        source: Option<anyhow::Error>,
        identifiable: bool,
    ) -> Self {
        let identifier = if identifiable {
            Some(Uuid::new_v4())
        } else {
            None
        };

        let result = Self {
            message,
            status_code,
            identifier,
            source,
        };

        if identifiable {
            if status_code.is_server_error() {
                event!(Level::ERROR, "{result}");
            } else {
                event!(Level::WARN, "{result}");
            };
        }

        result
    }

    pub fn new_500(source: anyhow::Error) -> Self {
        Self::new(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(source),
            true,
        )
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut log = format!("Status Code: {}.", self.status_code);

        if let Some(identifier) = &self.identifier {
            log = format!("{log} Identifier: {identifier}.");
        }

        log = format!("{log} {}.", self.message);

        if let Some(source) = &self.source {
            write!(f, "{log} Source Error: {source}")
        } else {
            write!(f, "{log}")
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|s| s as _)
    }
}

/// This enables using `.to_500()?` on functions that return `Result<_, anyhow::Error>` to turn them into
/// `Result<_, AppError>`. Only use this when no status code or message is intended. This will
/// default to a 500 status code and a generic message.
pub trait To500<T> {
    #[allow(clippy::missing_errors_doc)]
    fn to_500(self) -> Result<T, AppError>;
}
impl<T, E: Into<anyhow::Error>> To500<T> for Result<T, E> {
    fn to_500(self) -> Result<T, AppError> {
        self.map_err(|e| AppError::new_500(e.into()))
    }
}
