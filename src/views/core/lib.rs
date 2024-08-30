#![allow(clippy::missing_errors_doc)]

pub mod error;
pub mod footer;
pub mod header;
pub mod index;
pub mod not_found;

use std::convert::identity;
#[cfg(debug_assertions)]
use std::sync::RwLock;

#[cfg(debug_assertions)]
use hotwatch::{Event, EventKind, Hotwatch};

use anyhow::Result;
use lazy_static::lazy_static;
#[cfg(not(debug_assertions))]
use minify_html::{minify, Cfg};
use serde::Serialize;
use tera::Tera;
use tracing::{event, Level};

macro_rules! templates_dir {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates")
    };
}

#[cfg(not(debug_assertions))]
lazy_static! {
    static ref MINIFY_CONFIG: Cfg = {
        let mut cfg = Cfg::new();
        cfg.do_not_minify_doctype = true;
        cfg.minify_js = true;
        cfg.minify_css = true;
        cfg.keep_comments = false;

        cfg
    };
    static ref TERA: Tera =
        Tera::new(concat!(templates_dir!(), "/**/*.html")).unwrap();
}

#[cfg(debug_assertions)]
lazy_static! {
    static ref TERA: RwLock<Tera> =
        Tera::new(concat!(templates_dir!(), "/**/*.html"))
            .unwrap()
            .into();
    static ref HOTWATCH: Hotwatch = {
        use std::time::Duration;
        let mut hotwatch =
            Hotwatch::new_with_custom_delay(Duration::new(1, 0)).unwrap();
        hotwatch
            .watch(templates_dir!(), |event: Event| {
                match event.kind {
                    EventKind::Any | EventKind::Other => (),
                    _ => drop(TERA.write().unwrap().full_reload()),
                };
            })
            .unwrap();

        hotwatch
    };
}

pub trait AppTemplate: Serialize + Default {
    /// Renders the template with given path/name
    ///
    /// # Errors
    /// This method errors if Tera fails to render the template,
    /// or if the serialization with serde fails, or if
    /// (dev-only) tera full reload fails
    fn render(self, path: &'static str) -> Result<String>;
}

#[cfg(debug_assertions)]
pub fn setup_hotwatch() {
    let _ = &*HOTWATCH;
}

#[cfg(debug_assertions)]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> Result<String> {
    ctx.insert("env_is_dev", &true);
    let mteradev = TERA.read().unwrap();

    Ok(mteradev.render(path, &ctx)?)
}

#[cfg(not(debug_assertions))]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> Result<String> {
    ctx.insert("env_is_dev", &false);
    let raw = TERA.render(path, &ctx)?;
    let minified = minify(raw.as_bytes(), &MINIFY_CONFIG);
    let as_str = String::from_utf8(minified);
    if let Ok(res) = as_str {
        return Ok(res);
    }
    // Unreachable because `minify` will always return a valid UTF-8 string
    // given the input is also a valid UTF-8 string.
    unreachable!()
}

impl<T: Serialize + Default> AppTemplate for T {
    fn render(self, path: &'static str) -> Result<String> {
        // this is done in 2 steps since Context::from_serialize() calls .map_err
        // on a recoverable error
        let ctx_json = serde_json::to_value(self)?;
        let ctx = tera::Context::from_value(ctx_json)
            .map_or_else(|_| tera::Context::new(), identity);

        event!(Level::DEBUG, "render context: {:?}", ctx);

        render_internal(path, ctx)
    }
}
