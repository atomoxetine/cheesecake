use super::MinifyTemplate;
use crate::utils::custom_errors::app_error::AppError;

#[derive(askama::Template)]
#[template(path = "error.html")]
struct Template {
    message: String,
    status_code: u16,
    id: Option<String>,
    curr_date: String,
}

pub fn render(app_error: AppError) -> String {
    let templ = Template {
        message: app_error.message,
        status_code: app_error.status_code.as_u16(),
        id: app_error.identifier.map(|i| i.to_string()),
        curr_date: chrono::Local::now().to_rfc2822(),
    };
    match templ.render_minify() {
        Ok(html) => html,
        // TODO: This view CANNOT fail! Create wrapper that automatically returns hardcoded error page on error.
        Err(_) => todo!(),
    }
}
