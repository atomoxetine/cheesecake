use anyhow::Result;
use serde::Serialize;

use super::AppTemplate;

#[derive(Serialize, Default)]
struct Template;

pub fn render() -> Result<String> {
    Template.render("not_found.html")
}

#[test]
fn test() {
    assert!(Template.render("not_found.html").is_ok());
}
