use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    domain::aggregate::{
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    infrastructure::circle_repository::CircleRepository,
    usecase::{
        create_circle::{CreateCircleInput, CreateCircleUsecase},
        fetch_circle::FetchCircleUsecase,
        param::{
            create_circle::CreateCircleParam, fetch_circle::FetchCircleParam,
            update_circle::UpdateCircleParam,
        },
        update_circle::UpdateCircleUsecase,
    },
    AppState,
};

pub async fn handle_get(State(state): State<AppState>) -> String {
    println!("counter: {}", state.counter);
    println!("GET / html");
    "Hello, World!".to_string()
}

pub async fn handle_create_circle(Query(param): Query<CreateCircleParam>) -> impl IntoResponse {
    let grade = match param.owner_grade.as_str() {
        "first" => Grade::First,
        "second" => Grade::Second,
        "third" => Grade::Third,
        "fourth" => Grade::Fourth,
        _ => unimplemented!("error"),
    };

    let major = match param.owner_major.as_str() {
        "ComputerScience" => Major::ComputerScience,
        "Economics" => Major::Economics,
        "Law" => Major::Law,
        "Art" => Major::Art,
        "Music" => Major::Music,
        _ => Major::Other,
    };

    let owner = Member::new(param.owner_name, param.owner_age, grade, major);
    let circle_circle_input = CreateCircleInput {
        id: param.id,
        circle_name: param.circle_name,
        owner,
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
    let fetch_circle_input = FetchCircleParam { id: param.id };
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
    Query(param): Query<UpdateCircleParam>,
) -> impl IntoResponse {
    let update_circle_input = UpdateCircleParam {
        id: path.id,
        circle_name: param.circle_name,
        capacity: param.capacity,
    };
    let mut usecase = UpdateCircleUsecase::new(CircleRepository::new());

    usecase
        .execute(update_circle_input)
        .map_err(|e| e.to_string())
}
