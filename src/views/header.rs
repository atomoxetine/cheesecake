use super::MinifyTemplate;

#[derive(askama::Template)]
#[template(path = "header.html")]
struct Template;

#[allow(clippy::missing_errors_doc)]
pub fn render() -> Result<String, askama::Error> {
    Template.render_minify()
}
