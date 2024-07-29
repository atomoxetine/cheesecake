use askama::Template;

use super::MinifyTemplate;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    header: String
}

pub fn render() -> Result<String, askama::Error> {
    let header = super::header::render()?;
    IndexTemplate { header }.render_minify()
}
