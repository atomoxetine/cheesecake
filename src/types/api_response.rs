use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<ApiErrorLog>,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorLog {
    pub message: String,
    pub identifier: Option<String>,
    pub time: String,
}

impl<T> ApiResponse<T> {
    pub const fn success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    #[must_use]
    pub fn error(message: String, identifier: Option<Uuid>) -> Self {
        let error_log = ApiErrorLog {
            message,
            identifier: identifier.map(|i| i.to_string()),
            time: chrono::Utc::now()
                .to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
        };
        Self {
            data: None,
            error: Some(error_log),
        }
    }
}

impl<T> From<T> for ApiResponse<T> {
    fn from(data: T) -> Self {
        Self::success(data)
    }
}
