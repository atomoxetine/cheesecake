use std::marker::PhantomData;

use crate::types::api_response::ApiResponse;
use crate::views::error::render;
use axum::{
    response::{Html, IntoResponse, Response},
    Json,
};

use super::app_error::AppError;

pub struct ErrResponse<Kind>(AppError, PhantomData<Kind>);
impl<Kind> ErrResponse<Kind> {
    fn new(source: AppError) -> Self {
        Self(source, PhantomData)
    }
}

pub struct ViewKind;
pub type ViewErrResponse = ErrResponse<ViewKind>;
impl<T: Into<AppError>> From<T> for ViewErrResponse {
    fn from(source: T) -> Self {
        Self::new(source.into())
    }
}

pub struct JsonKind;
pub type ApiErrResponse = ErrResponse<JsonKind>;
impl<T: Into<AppError>> From<T> for ApiErrResponse {
    fn from(source: T) -> Self {
        Self::new(source.into())
    }
}

// Tell axum how to convert `ViewErrResponse` into a response.
impl IntoResponse for ViewErrResponse {
    fn into_response(self) -> Response {
        let ErrResponse(source, _) = self;
        (source.status_code, Html(render(source))).into_response()
    }
}

// Tell axum how to convert `ApiErrResponse` into a response.
impl IntoResponse for ApiErrResponse {
    fn into_response(self) -> Response {
        let ErrResponse(
            AppError {
                message,
                status_code,
                identifier,
                ..
            },
            _,
        ) = self;
        let api_response = ApiResponse::<()>::error(message, identifier);
        (status_code, Json(api_response)).into_response()
    }
}
