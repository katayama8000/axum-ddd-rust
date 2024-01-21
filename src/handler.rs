use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    infrastructure::circle_repository::CircleRepository,
    usecase::{
        create_circle::{CreateCircleInput, CreateCircleService},
        fetch_circle::{FetchCircleInput, FetchCircleService},
    },
    AppState,
};

pub async fn handle_get(State(state): State<AppState>) -> String {
    println!("counter: {}", state.counter);
    println!("GET / html");
    "Hello, World!".to_string()
}

#[derive(Debug, Deserialize)]
pub struct CreateCircleInputParam {
    id: usize,
    circle_name: String,
    owner_name: String,
    capacity: usize,
}

pub async fn handle_create_circle(
    Query(param): Query<CreateCircleInputParam>,
) -> impl IntoResponse {
    let circle_circle_input = CreateCircleInput {
        id: param.id,
        circle_name: param.circle_name,
        owner_name: param.owner_name,
        capacity: param.capacity,
    };
    let application_service = CreateCircleService::new(CircleRepository::new());
    application_service.execute(circle_circle_input);
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: usize,
}

pub async fn handle_fetch_circle(Path(param): Path<FetchCircleInputParam>) -> impl IntoResponse {
    let fetch_circle_input = FetchCircleInput { id: param.id };
    let application_service = FetchCircleService::new(CircleRepository::new());
    application_service.execute(fetch_circle_input);
}
