use axum::{Router, Server};
use dotenv::dotenv;
use std::env;

use server::routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let addr = format!("{}:{}", host, port);

    let app = Router::new().nest("/api", routes());

    tracing::info!("listening on http://{}", addr);
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
