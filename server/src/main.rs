use axum::{routing::get, Router, Server};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let addr = format!("{}:{}", host, port);

    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    tracing::info!("listening on {}", addr);
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
