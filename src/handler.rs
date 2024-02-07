use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    infrastructure::circle_repository::CircleRepository,
    usecase::{
        create_circle::{CreateCircleInput, CreateCircleUsecase},
        fetch_circle::{FetchCircleInput, FetchCircleUsecase},
        update_circle::{UpdateCircleInput, UpdateCircleUsecase},
    },
    AppState,
};

pub async fn handle_get(State(state): State<AppState>) -> String {
    println!("counter: {}", state.counter);
    println!("GET / html");
    "Hello, World!".to_string()
}

pub async fn handle_create_circle(Query(param): Query<CreateCircleInput>) -> impl IntoResponse {
    let circle_circle_input = CreateCircleInput::new(
        param.circle_name,
        param.capacity,
        param.owner_name,
        param.owner_age,
        param.owner_grade,
        param.owner_major,
    );
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
    let fetch_circle_input = FetchCircleInput::new(param.id);
    let usecase = FetchCircleUsecase::new(CircleRepository::new());
    // return json
    let _ = usecase.execute(fetch_circle_input);
}

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInputParam {
    id: usize,
}
pub async fn handle_update_circle(
    Path(path): Path<UpdateCircleInputParam>,
    Query(param): Query<UpdateCircleInput>,
) -> impl IntoResponse {
    let update_circle_input = UpdateCircleInput::new(path.id, param.circle_name, param.capacity);
    let mut usecase = UpdateCircleUsecase::new(CircleRepository::new());

    usecase
        .execute(update_circle_input)
        .map_err(|e| e.to_string())
}
