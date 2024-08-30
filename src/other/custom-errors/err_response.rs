use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use std::marker::PhantomData;
use types::api;
use uuid::Uuid;
use views::error;

pub struct ErrResponse<Kind> {
    pub message: String,
    pub status_code: StatusCode,
    pub identifier: Option<Uuid>,
    _kind: PhantomData<Kind>,
}
impl<Kind> ErrResponse<Kind> {
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
            _kind: PhantomData,
        }
    }
}

pub struct HtmlKind;
pub struct JsonKind;

// Tell axum how to convert `ErrResponse<HtmlKind>` into a response.
impl IntoResponse for ErrResponse<HtmlKind> {
    fn into_response(self) -> Response {
        let Self {
            message,
            status_code,
            identifier,
            ..
        } = self;
        let html = Html(error::render(
            status_code.as_u16(),
            message,
            identifier.map(|uuid| uuid.to_string()),
            true,
        ));
        (status_code, html).into_response()
    }
}

// Tell axum how to convert `ErrResponse<JsonKind>` into a response.
impl IntoResponse for ErrResponse<JsonKind> {
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

pub type HtmlResult = Result<Response, ErrResponse<HtmlKind>>;
pub type JsonResult = Result<Response, ErrResponse<JsonKind>>;

#[inline]
#[allow(clippy::missing_errors_doc)]
pub fn res<Kind>(
    res: impl IntoResponse,
) -> Result<Response, ErrResponse<Kind>> {
    Ok(res.into_response())
}
