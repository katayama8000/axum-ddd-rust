use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

use tokio;

#[tokio::main]
async fn main() -> Result<(), StatusCode> {
    let app = Router::new().route("/", get(handler_get));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handler_get() -> String {
    println!("GET / html");
    "Hello, World!".to_string()
}
