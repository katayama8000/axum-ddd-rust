mod domain;
pub mod infrastructure;
pub mod usecase;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Router,
};

use infrastructure::circle_repository::CircleRepository;
use tokio;
use usecase::create_circle::{CreateCircleInput, CreateCircleService};

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
        .route("/create", get(handler_plus))
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

async fn handler_sample(num: i32) {
    println!("GET /sample {}", num);
}

struct CreateCircleInputParam {
    id: usize,
    circle_name: String,
    owner_name: String,
    capacity: usize,
}

async fn create_circle(Query(param): Query<CreateCircleInputParam>) {
    let circle_circle_input = CreateCircleInput {
        id: param.id,
        circle_name: param.circle_name,
        owner_name: param.owner_name,
        capacity: param.capacity,
    };
    let application_service = CreateCircleService::new(CircleRepository::new());
    application_service.execute(circle_circle_input)
}
