use axum::{http::StatusCode, response::IntoResponse};

use axum::response::Json;
use axum_extra::extract::WithRejection;

use axum::Json as JsonExt;
use serde::{Deserialize, Serialize};
use types::api::Response;

use crate::app_rejection::WithJsonRejection;

pub async fn get(
    WithRejection(JsonExt(json), _): WithJsonRejection<JsonExt<Input>>,
) -> impl IntoResponse {
    let response = Response::success(Output {
        example_output: json.example_input,
    });
    (StatusCode::OK, Json(response)).into_response()
}

#[derive(Deserialize)]
pub struct Input {
    example_input: String,
}

#[derive(Serialize)]
pub struct Output {
    example_output: String,
}
