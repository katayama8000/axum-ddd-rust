use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    infrastructure::circle_repository::CircleRepository,
    usecase::{
        create_circle::{CreateCircleInput, CreateCircleUsecase},
        fetch_circle::FetchCircleUsecase,
        input::{create_circle::CreateCircleInputParam, fetch_circle::FetchCircleInput},
    },
    AppState,
};

pub async fn handle_get(State(state): State<AppState>) -> String {
    println!("counter: {}", state.counter);
    println!("GET / html");
    "Hello, World!".to_string()
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
    let mut usecase = CreateCircleUsecase::new(CircleRepository::new());
    usecase
        .execute(circle_circle_input)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: usize,
}

pub async fn handle_fetch_circle(Path(param): Path<FetchCircleInputParam>) -> impl IntoResponse {
    let fetch_circle_input = FetchCircleInput { id: param.id };
    let usecase = FetchCircleUsecase::new(CircleRepository::new());
    // return json
    let _ = usecase.execute(fetch_circle_input);
}
