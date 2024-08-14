pub mod controllers;
pub mod repositories;
pub mod services;

use crate::{
    types::api_response::ApiResponse,
    utils::custom_errors::err_response::{ApiErrResponse, ViewErrResponse},
};
use axum::{response::Html, Json};
use axum_extra::extract::WithRejection;

pub type ViewResult = Result<Html<String>, ViewErrResponse>;
pub type ApiResult<T> = Result<Json<ApiResponse<T>>, ApiErrResponse>;

pub type WithViewRejection<T> = WithRejection<T, ViewErrResponse>;
pub type WithApiRejection<T> = WithRejection<T, ApiErrResponse>;
