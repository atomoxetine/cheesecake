use axum::http::StatusCode;

use axum::response::Json;
use axum_extra::extract::WithRejection;
use custom_errors::err_response::{res, JsonResult};
use types::api;

use axum::Json as JsonExt;
use serde::{Deserialize, Serialize};

use custom_errors::app_rejection::WithJsonRejection;

pub async fn get(
    WithRejection(JsonExt(json), _): WithJsonRejection<JsonExt<Input>>,
) -> JsonResult {
    let response = api::Response::success(Output {
        example_output: json.example_input,
    });
    res((StatusCode::OK, Json(response)))
}

#[derive(Deserialize)]
pub struct Input {
    example_input: String,
}

#[derive(Serialize)]
pub struct Output {
    example_output: String,
}
