mod domain;
pub mod infrastructure;
pub mod usecase;

use axum::{extract::State, http::StatusCode, routing::get, Router};

use tokio;

#[derive(Clone)]
struct AppState {
    counter: usize,
}

#[tokio::main]
async fn main() -> Result<(), StatusCode> {
    let state = AppState { counter: 0 };
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/plus", get(handler_plus))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handler_get(State(state): State<AppState>) -> String {
    println!("counter: {}", state.counter);
    println!("GET / html");
    "Hello, World!".to_string()
}

async fn handler_plus(State(mut state): State<AppState>) {
    state.counter += 1;
}
