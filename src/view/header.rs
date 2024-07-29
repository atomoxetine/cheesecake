use askama::Template;

use super::MinifyTemplate;

#[derive(Template)]
#[template(path = "header.html")]
pub struct HeaderTemplate {}

pub fn render() -> Result<String, askama::Error> {
    HeaderTemplate {}.render_minify()
}
