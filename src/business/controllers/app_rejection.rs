use axum::extract::rejection::{
    BytesRejection, ExtensionRejection, FailedToBufferBody, FormRejection,
    HostRejection, JsonRejection, MatchedPathRejection, NestedPathRejection,
    PathRejection, QueryRejection, RawFormRejection, RawPathParamsRejection,
    StringRejection,
};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::WithRejection;
use types::api::ErrResponse;

#[derive(Debug)]
pub struct AppRejection {
    pub message: String,
    pub status_code: StatusCode,
}

impl AppRejection {
    #[must_use]
    pub const fn new(message: String, status_code: StatusCode) -> Self {
        Self {
            message,
            status_code,
        }
    }
}

// Tell axum how to convert `AppRejection` into a response.
impl IntoResponse for AppRejection {
    fn into_response(self) -> Response {
        let Self {
            message,
            status_code,
        } = self;
        let api_failure = ErrResponse::error(status_code, message);
        api_failure.into_response()
    }
}

pub type WithJsonRejection<T> = WithRejection<T, AppRejection>;

macro_rules! from_rejection {
    ($from:ty) => {
        impl From<$from> for AppRejection {
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
