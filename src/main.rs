use crate::handler::{handle_create_circle, handle_fetch_circle, handle_get};

mod domain;
mod handler;
mod infrastructure;
mod usecase;

use axum::{
    routing::{get, post},
    Router,
};

#[derive(Clone)]
struct AppState {
    counter: usize,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let state = AppState { counter: 0 };
    let app = Router::new()
        .route("/", get(handle_get))
        .route("/circle/:id", get(handle_fetch_circle))
        .route("/circle", post(handle_create_circle))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
