mod content;
mod page;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { Html(page::page()) }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010")
        .await
        .expect("bind 127.0.0.1:3010");
    println!("html-auy demo: http://127.0.0.1:3010");
    axum::serve(listener, app).await.expect("server");
}
