use std::str::from_utf8;

use askama::Template;
use axum::response::Html;
use minify_html::{minify, Cfg};

pub trait MinifyTemplate {
    fn render_html(&self) -> Option<Html<String>>;
}

impl<T> MinifyTemplate for T
where
    T: Template,
{
    fn render_html(&self) -> Option<Html<String>> {
        let res = self.render().ok()?;
        Some(Html(
            from_utf8(minify(res.as_bytes(), &Cfg::new()).as_slice())
                .ok()?
                .to_string(),
        ))
    }
}
