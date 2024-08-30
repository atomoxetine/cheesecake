use anyhow::Result;
use serde::Serialize;

use super::AppTemplate;

#[derive(Serialize, Default)]
struct Template;

pub fn render() -> Result<String> {
    Template.render("header.html")
}

#[test]
fn test() {
    assert!(Template.render("header.html").is_ok());
}
