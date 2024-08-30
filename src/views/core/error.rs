use std::convert::identity;

use serde::Serialize;

use super::AppTemplate;

#[derive(Serialize, Default)]
struct Template {
    message: String,
    status_code: u16,
    id: Option<String>,
    curr_date: String,
    show_back_anchor: bool,
}

#[must_use]
pub fn render(
    status_code: u16,
    message: String,
    id: Option<String>,
    show_back_anchor: bool,
) -> String {
    let templ = Template {
        message,
        status_code,
        id,
        curr_date: chrono::Local::now().to_rfc2822(),
        show_back_anchor,
    };
    templ.render("error.html").map_or_else(
        |_| {
            "An error has ocurred while rendering the Error Page. Whoops..."
                .to_string()
        },
        identity,
    )
}

#[test]
fn test() {
    assert!(Template::default().render("error.html").is_ok());
    let id = Some("cb910ce9-d611-4345-89f8-6399b836cf7b".to_string());
    let tmpl = Template {
        id,
        ..Default::default()
    };
    assert!(tmpl.render("error.html").is_ok());
}
