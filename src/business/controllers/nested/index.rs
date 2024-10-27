use axum::http::StatusCode;

use axum::response::Json;
use custom_errors::err_response::{res, JsonResult};
use environment::ENV;
use tracing::{event, Level};
use types::api;

use axum::http::{HeaderMap, HeaderValue};
use axum::Json as JsonExt;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::{CookieJar, WithRejection};
use cookie::time::Duration;
use serde::{Deserialize, Serialize};
use validator::Validate;

use custom_errors::app_rejection::WithJsonRejection;

pub async fn get(
    cookie_jar: CookieJar,
    WithRejection(JsonExt(json), _): WithJsonRejection<JsonExt<Signature>>,
) -> JsonResult {
    if cookie_jar.get("example_cookie").is_some() {
        return res(StatusCode::FORBIDDEN);
    }

    if let Err(e) = json.validate() {
        let errors = e
            .field_errors()
            .iter()
            .flat_map(|e| e.1.iter())
            .filter_map(|e| e.message.as_ref())
            .fold(String::new(), |acc, e| acc + e + "\n");

        let response = api::Response::<()>::error(
            format!("Whoops, validation errors! {errors}"),
            None,
        );
        return res((StatusCode::OK, Json(response)));
    };

    let cookie = Cookie::build(("example_cookie", "example"))
        .domain(&*ENV.domain)
        .path("/")
        .secure(false)
        .http_only(false)
        .max_age(Duration::days(30))
        .same_site(SameSite::None)
        .build();

    let cookie_jar = cookie_jar.add(cookie);

    let mut headers = HeaderMap::new();
    headers.insert("Example-Header", HeaderValue::from_static("example"));

    event!(Level::INFO, "Example text: {}", json.example_text);

    let response = api::Response::success(Return {
        example_return: "Success!".to_string(),
    });
    res((StatusCode::OK, cookie_jar, headers, Json(response)))
}

#[derive(Serialize)]
pub struct Return {
    example_return: String,
}

#[derive(Validate, Deserialize)]
pub struct Signature {
    #[validate(length(
        min = 6,
        max = 50,
        message = "example_text is 6 to 50 characters!"
    ))]
    example_text: String,
}
