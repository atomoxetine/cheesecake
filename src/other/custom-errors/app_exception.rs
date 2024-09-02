use std::fmt;

use axum::http::StatusCode;
use tracing::{event, Level};
use uuid::Uuid;

use super::err_response::ErrResponse;

#[derive(Debug)]
pub struct AppException {
    identifier: Uuid,
    pub source: anyhow::Error,
}

impl AppException {
    pub fn new(source: anyhow::Error) -> Self {
        let result = Self {
            identifier: Uuid::new_v4(),
            source,
        };

        event!(Level::ERROR, "{result}");

        result
    }

    #[must_use]
    pub const fn identifier(&self) -> Uuid {
        self.identifier
    }
}

fn cut_trace(trace: &str) -> &str {
    trace
        .find("axum::handler::Handler")
        .and_then(|i| {
            let temp = trace.get(0..i)?;
            let idx = temp.rfind('\n')?;
            trace.get(0..idx)
        })
        .unwrap_or("[backtrace unavailable]")
}

impl fmt::Display for AppException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.identifier;
        let source = &self.source;
        let backtrace = self.source.backtrace().to_string();
        write!(
      f,
      "INTERNAL SERVER ERROR! Identifier: {id}. Source Error: {source}. Backtrace:\n{}",
      cut_trace(&backtrace)
    )
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppException {
    fn from(err: E) -> Self {
        Self::new(err.into())
    }
}

impl<Kind, E: Into<AppException>> From<E> for ErrResponse<Kind> {
    fn from(source: E) -> Self {
        Self::new(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(source.into().identifier()),
        )
    }
}
