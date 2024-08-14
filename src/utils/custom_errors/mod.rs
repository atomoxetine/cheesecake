pub mod app_error;
pub mod err_response;
mod from_rejection;

use app_error::AppError;

// You can implement here any custom errors so you can use the operator
// `?` in your controllers to automatically convert them to AppError.

impl From<askama::Error> for AppError {
    fn from(value: askama::Error) -> Self {
        Self::new_500(value.into())
    }
}
