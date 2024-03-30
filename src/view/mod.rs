
pub mod index;

use askama::Template;

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate {
    title: String,
}
