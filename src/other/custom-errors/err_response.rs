use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use types::api;
use uuid::Uuid;

pub struct ErrResponse {
    pub message: String,
    pub status_code: StatusCode,
    pub identifier: Option<Uuid>,
}
impl ErrResponse {
    #[must_use]
    pub const fn new(
        message: String,
        status_code: StatusCode,
        identifier: Option<Uuid>,
    ) -> Self {
        Self {
            message,
            status_code,
            identifier,
        }
    }
}

// Tell axum how to convert `ErrResponse` into a response.
impl IntoResponse for ErrResponse {
    fn into_response(self) -> Response {
        let Self {
            message,
            status_code,
            identifier,
            ..
        } = self;
        let api_response = api::Response::<()>::error(message, identifier);
        (status_code, Json(api_response)).into_response()
    }
}

pub type JsonResult = Result<Response, ErrResponse>;

#[inline]
#[allow(clippy::missing_errors_doc)]
pub fn res(res: impl IntoResponse) -> Result<Response, ErrResponse> {
    Ok(res.into_response())
}
