use std::marker::PhantomData;

use axum::extract::rejection::{
    BytesRejection, ExtensionRejection, FailedToBufferBody, FormRejection,
    HostRejection, JsonRejection, MatchedPathRejection, NestedPathRejection,
    PathRejection, QueryRejection, RawFormRejection, RawPathParamsRejection,
    StringRejection,
};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use axum_extra::extract::WithRejection;

use types::api;
use views::error;

use super::err_response::{HtmlKind, JsonKind};

#[derive(Debug)]
pub struct AppRejection<Kind> {
    pub message: String,
    pub status_code: StatusCode,
    _kind: PhantomData<Kind>,
}

impl<Kind> AppRejection<Kind> {
    #[must_use]
    pub const fn new(message: String, status_code: StatusCode) -> Self {
        Self {
            message,
            status_code,
            _kind: PhantomData,
        }
    }
}

// Tell axum how to convert `AppRejection<HtmlKind>` into a response.
impl IntoResponse for AppRejection<HtmlKind> {
    fn into_response(self) -> Response {
        let Self {
            message,
            status_code,
            ..
        } = self;
        let html =
            Html(error::render(status_code.as_u16(), message, None, false));
        (status_code, html).into_response()
    }
}

// Tell axum how to convert `AppRejection<JsonKind>` into a response.
impl IntoResponse for AppRejection<JsonKind> {
    fn into_response(self) -> Response {
        let Self {
            message,
            status_code,
            ..
        } = self;
        let api_response = api::Response::<()>::error(message, None);
        (status_code, Json(api_response)).into_response()
    }
}

pub type WithHtmlRejection<T> = WithRejection<T, AppRejection<HtmlKind>>;
pub type WithJsonRejection<T> = WithRejection<T, AppRejection<JsonKind>>;

macro_rules! from_rejection {
    ($from:ty) => {
        impl From<$from> for AppRejection<HtmlKind> {
            fn from(value: $from) -> Self {
                Self::new(value.body_text(), value.status())
            }
        }
        impl From<$from> for AppRejection<JsonKind> {
            fn from(value: $from) -> Self {
                Self::new(value.body_text(), value.status())
            }
        }
    };
}

from_rejection!(FormRejection);
from_rejection!(HostRejection);
from_rejection!(JsonRejection);
from_rejection!(PathRejection);
from_rejection!(BytesRejection);
from_rejection!(QueryRejection);
from_rejection!(StringRejection);
from_rejection!(RawFormRejection);
from_rejection!(ExtensionRejection);
from_rejection!(FailedToBufferBody);
from_rejection!(NestedPathRejection);
from_rejection!(MatchedPathRejection);
from_rejection!(RawPathParamsRejection);
