use super::MinifyTemplate;

#[derive(askama::Template)]
#[template(path = "index.html")]
struct Template {
    header: String,
}

#[allow(clippy::missing_errors_doc)]
pub fn render() -> Result<String, askama::Error> {
    let header = super::header::render()?;
    Template { header }.render_minify()
}
