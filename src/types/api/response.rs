use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub error: Option<ErrorLog>,
}

#[derive(Debug, Serialize)]
pub struct ErrorLog {
    pub message: String,
    pub identifier: Option<String>,
    pub time: String,
}

impl<T> Response<T> {
    pub const fn success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    #[must_use]
    pub fn error(message: String, identifier: Option<Uuid>) -> Self {
        let error_log = ErrorLog {
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

impl<T> From<T> for Response<T> {
    fn from(data: T) -> Self {
        Self::success(data)
    }
}
