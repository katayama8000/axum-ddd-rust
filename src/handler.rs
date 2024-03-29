use std::env;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{
    usecase::{
        create_circle::{CreateCircleInput, CreateCircleOutput, CreateCircleUsecase},
        fetch_circle::{FetchCircleInput, FetchCircleOutput, FetchCircleUsecase, MemberOutput},
        update_circle::{UpdateCircleInput, UpdateCircleOutPut, UpdateCircleUsecase},
    },
    AppState,
};

pub async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleRequestBody {
    pub circle_name: String,
    pub capacity: usize,
    pub owner_name: String,
    pub owner_age: usize,
    pub owner_grade: usize,
    pub owner_major: String,
}

impl std::convert::From<CreateCircleRequestBody> for CreateCircleInput {
    fn from(
        CreateCircleRequestBody {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }: CreateCircleRequestBody,
    ) -> Self {
        CreateCircleInput::new(
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        )
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleResponseBody {
    pub circle_id: usize,
    pub owner_id: usize,
}

impl std::convert::From<CreateCircleOutput> for CreateCircleResponseBody {
    fn from(
        CreateCircleOutput {
            circle_id,
            owner_id,
        }: CreateCircleOutput,
    ) -> Self {
        CreateCircleResponseBody {
            circle_id,
            owner_id,
        }
    }
}

pub async fn handle_create_circle(
    State(state): State<AppState>,
    Json(body): Json<CreateCircleRequestBody>,
) -> Result<Json<CreateCircleResponseBody>, String> {
    let circle_circle_input = CreateCircleInput::from(body);
    let mut usecase = CreateCircleUsecase::new(state.circle_repository);
    usecase
        .execute(circle_circle_input)
        .map(CreateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetcheCircleResponseBody {
    pub circle_id: usize,
    pub circle_name: String,
    pub capacity: usize,
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

impl std::convert::From<FetchCircleOutput> for FetcheCircleResponseBody {
    fn from(
        FetchCircleOutput {
            circle_id,
            circle_name,
            capacity,
            owner,
            members,
        }: FetchCircleOutput,
    ) -> Self {
        FetcheCircleResponseBody {
            circle_id,
            circle_name,
            capacity,
            owner,
            members,
        }
    }
}

pub async fn handle_fetch_circle(
    State(state): State<AppState>,
    Path(param): Path<FetchCircleInputParam>,
) -> Result<Json<FetcheCircleResponseBody>, String> {
    let fetch_circle_input = FetchCircleInput::new(param.id);
    let usecase = FetchCircleUsecase::new(state.circle_repository);
    usecase
        .execute(fetch_circle_input)
        .map(FetcheCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInputParam {
    id: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateCircleRequestBody {
    pub circle_name: Option<String>,
    pub capacity: Option<usize>,
}

impl UpdateCircleRequestBody {
    pub fn convert_to_input(self, id: usize) -> UpdateCircleInput {
        UpdateCircleInput::new(id, self.circle_name, self.capacity)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateCircleResponseBody {
    pub id: usize,
}

impl std::convert::From<UpdateCircleOutPut> for UpdateCircleResponseBody {
    fn from(UpdateCircleOutPut { id }: UpdateCircleOutPut) -> Self {
        UpdateCircleResponseBody { id }
    }
}

pub async fn handle_update_circle(
    State(state): State<AppState>,
    Path(path): Path<UpdateCircleInputParam>,
    Json(body): Json<UpdateCircleRequestBody>,
) -> Result<Json<UpdateCircleResponseBody>, String> {
    let update_circle_input = body.convert_to_input(path.id);
    let mut usecase = UpdateCircleUsecase::new(state.circle_repository);

    usecase
        .execute(update_circle_input)
        .map(UpdateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}
