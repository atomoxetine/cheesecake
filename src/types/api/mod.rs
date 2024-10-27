use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use tracing::{event, Level};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub status_code: u16,
    pub data: Option<T>,
    pub error: Option<ErrorLog>,
}
#[derive(Serialize)]
pub struct Unit;
pub type ErrResponse = Response<Unit>;

#[derive(Debug, Serialize)]
pub struct ErrorLog {
    pub message: String,
    pub identifier: Option<String>,
}

impl<T: Serialize> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            status_code: StatusCode::OK.as_u16(),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(status_code: StatusCode, message: String) -> Self {
        let identifier = if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            let uuid = Uuid::new_v4().to_string();
            event!(Level::ERROR, "{uuid} // {message}");
            Some(uuid)
        } else {
            None
        };

        Self {
            status_code: status_code.as_u16(),
            data: None,
            error: Some(ErrorLog {
                message,
                identifier,
            }),
        }
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
impl<T: Serialize> From<T> for Response<T> {
    fn from(data: T) -> Self {
        Self::success(data)
    }
}

impl<E: Into<anyhow::Error>> From<E> for ErrResponse {
    fn from(source: E) -> Self {
        let source: anyhow::Error = source.into();
        Self::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error! {source}"),
        )
    }
}
