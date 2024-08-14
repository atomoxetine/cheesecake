pub mod error;
pub mod header;
pub mod index;
pub mod nested;
pub mod not_found;

use askama::Template;
use lazy_static::lazy_static;
use minify_html::{minify, Cfg};

pub trait MinifyTemplate {
    /// # Errors
    ///
    /// Will return `Err` if the minification fails on the template engine.
    fn render_minify(&self) -> Result<String, askama::Error>;
}

lazy_static! {
    static ref MINIFY_CONFIG: Cfg = {
        let mut cfg = Cfg::new();
        cfg.do_not_minify_doctype = true;
        cfg.minify_js = true;
        cfg.minify_css = true;
        cfg.keep_comments = false;

        cfg
    };
}

impl<T> MinifyTemplate for T
where
    T: Template,
{
    fn render_minify(&self) -> Result<String, askama::Error> {
        let raw = self.render()?;
        #[allow(clippy::unwrap_used)]
        Ok(std::str::from_utf8(
            minify(raw.as_bytes(), &MINIFY_CONFIG).as_slice(),
        )
        .unwrap()
        .to_owned())
    }
}
