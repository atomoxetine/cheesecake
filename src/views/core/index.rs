use anyhow::Result;
use serde::Serialize;

use super::AppTemplate;

#[derive(Serialize, Default)]
struct Template {
    header: String,
    footer: String,
}

pub fn render() -> Result<String> {
    let header = super::header::render()?;
    let footer = super::footer::render()?;
    Template { header, footer }.render("index.html")
}

#[test]
fn test() {
    assert!(Template::default().render("index.html").is_ok());
}
