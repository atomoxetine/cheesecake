use anyhow::Result;
use serde::Serialize;

use super::AppTemplate;

#[derive(Serialize, Default)]
struct Template;

pub fn render() -> Result<String> {
    Template.render("footer.html")
}

#[test]
fn test() {
    assert!(Template.render("footer.html").is_ok());
}
