use askama::Template;
use axum::{response::Html, routing::get, Router};
use config::PORT;
use dotenv;
use utils::minify_template::MinifyTemplate;

pub mod config;
pub mod utils;

#[derive(Template)]
#[template(path = "test.html")]
struct TestTemplate<'a> {
    test: &'a str,
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let host = format!("0.0.0.0:{}", *PORT);
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    TestTemplate {
        test: "hello world",
    }.render_html().unwrap()
}

