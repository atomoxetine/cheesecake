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
        let minified = minify(raw.as_bytes(), &MINIFY_CONFIG);
        let as_str = std::str::from_utf8(minified.as_slice());
        if let Ok(res) = as_str {
            return Ok(res.to_owned());
        }
        // Unreachable because `minify` will always return a valid UTF-8 string
        // given the input is also a valid UTF-8 string.
        unreachable!()
    }
}
