use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::env;
use usecase::{
    create_circle::{CreateCircleInput, CreateCircleOutput, CreateCircleUsecase},
    fetch_all_circle::FetchAllCircleUsecase,
    fetch_circle::{FetchCircleInput, FetchCircleOutput, FetchCircleUsecase, MemberOutput},
    fetch_me::{FetchMeInput, FetchMeOutput, FetchMeUsecase},
    sign_in::{SignInInput, SignInOutput, SignInUsecase},
    sign_up::{SignUpInput, SignUpOutput, SignUpUsecase},
    update_circle::{UpdateCircleInput, UpdateCircleOutPut, UpdateCircleUsecase},
};

use crate::{app::AppState, error::AppError, extractor::AuthUser};

pub(crate) async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleRequestBody {
    pub circle_name: String,
    pub capacity: i16,
    pub owner_name: String,
    pub owner_age: i16,
    pub owner_grade: i16,
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
        CreateCircleInput {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleResponseBody {
    pub circle_id: String,
    pub owner_id: String,
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

pub(crate) async fn handle_create_circle(
    State(state): State<AppState>,
    Json(body): Json<CreateCircleRequestBody>,
) -> Result<Json<CreateCircleResponseBody>, String> {
    let circle_circle_input = CreateCircleInput::from(body);
    let mut usecase =
        CreateCircleUsecase::new(state.circle_repository, state.circle_duplicate_checker);
    usecase
        .execute(circle_circle_input)
        .await
        .map(CreateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetcheCircleResponseBody {
    pub circle_id: String,
    pub circle_name: String,
    pub capacity: i16,
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

pub(crate) async fn handle_fetch_circle(
    State(state): State<AppState>,
    Path(param): Path<FetchCircleInputParam>,
) -> Result<Json<FetcheCircleResponseBody>, String> {
    let fetch_circle_input = FetchCircleInput::new(param.id);
    let usecase = FetchCircleUsecase::new(state.circle_repository);
    usecase
        .execute(fetch_circle_input)
        .await
        .map(FetcheCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

pub(crate) async fn handle_fetch_all(State(state): State<AppState>) -> impl IntoResponse {
    let usecase = FetchAllCircleUsecase::new(state.circle_repository);
    let circles = usecase.execute().await;
    tracing::info!("circles: {:?}", circles);
    (StatusCode::OK).into_response()
}

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInputParam {
    id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateCircleRequestBody {
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
}

impl UpdateCircleRequestBody {
    pub fn convert_to_input(self, id: String) -> UpdateCircleInput {
        UpdateCircleInput::new(id, self.circle_name, self.capacity)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateCircleResponseBody {
    pub circle_id: String,
}

impl std::convert::From<UpdateCircleOutPut> for UpdateCircleResponseBody {
    fn from(UpdateCircleOutPut { circle_id }: UpdateCircleOutPut) -> Self {
        UpdateCircleResponseBody { circle_id }
    }
}

pub(crate) async fn handle_update_circle(
    State(state): State<AppState>,
    Path(path): Path<UpdateCircleInputParam>,
    Json(body): Json<UpdateCircleRequestBody>,
) -> Result<Json<UpdateCircleResponseBody>, String> {
    let update_circle_input = body.convert_to_input(path.id.to_string());
    let mut usecase = UpdateCircleUsecase::new(state.circle_repository);

    usecase
        .execute(update_circle_input)
        .await
        .map(UpdateCircleResponseBody::from)
        .map(Json)
        .map_err(|e| e.to_string())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SignUpRequestBody {
    pub email: String,
    pub password: String,
}

impl std::convert::From<SignUpRequestBody> for SignUpInput {
    fn from(SignUpRequestBody { email, password }: SignUpRequestBody) -> Self {
        SignUpInput { email, password }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SignUpResponseBody {
    pub user_id: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl std::convert::From<SignUpOutput> for SignUpResponseBody {
    fn from(
        SignUpOutput {
            user_id,
            access_token,
            expires_in,
        }: SignUpOutput,
    ) -> Self {
        SignUpResponseBody {
            user_id,
            access_token,
            token_type: "Bearer".to_string(),
            expires_in,
        }
    }
}

pub(crate) async fn handle_sign_up(
    State(state): State<AppState>,
    Json(body): Json<SignUpRequestBody>,
) -> Result<(StatusCode, Json<SignUpResponseBody>), AppError> {
    let mut usecase = SignUpUsecase::new(
        state.user_repository,
        state.password_hasher,
        state.access_token_issuer,
    );
    let output = usecase.execute(SignUpInput::from(body)).await?;

    Ok((StatusCode::CREATED, Json(SignUpResponseBody::from(output))))
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SignInRequestBody {
    pub email: String,
    pub password: String,
}

impl std::convert::From<SignInRequestBody> for SignInInput {
    fn from(SignInRequestBody { email, password }: SignInRequestBody) -> Self {
        SignInInput { email, password }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SignInResponseBody {
    pub user_id: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl std::convert::From<SignInOutput> for SignInResponseBody {
    fn from(
        SignInOutput {
            user_id,
            access_token,
            expires_in,
        }: SignInOutput,
    ) -> Self {
        SignInResponseBody {
            user_id,
            access_token,
            token_type: "Bearer".to_string(),
            expires_in,
        }
    }
}

pub(crate) async fn handle_sign_in(
    State(state): State<AppState>,
    Json(body): Json<SignInRequestBody>,
) -> Result<Json<SignInResponseBody>, AppError> {
    let usecase = SignInUsecase::new(
        state.user_repository,
        state.password_hasher,
        state.access_token_issuer,
    );
    let output = usecase.execute(SignInInput::from(body)).await?;

    Ok(Json(SignInResponseBody::from(output)))
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetchMeResponseBody {
    pub user_id: String,
    pub email: String,
}

impl std::convert::From<FetchMeOutput> for FetchMeResponseBody {
    fn from(FetchMeOutput { user_id, email }: FetchMeOutput) -> Self {
        FetchMeResponseBody { user_id, email }
    }
}

/// Taking `AuthUser` is what protects this route: the extractor rejects the request with 401
/// before the handler body runs.
pub(crate) async fn handle_fetch_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<FetchMeResponseBody>, AppError> {
    let usecase = FetchMeUsecase::new(state.user_repository);
    let output = usecase
        .execute(FetchMeInput {
            user_id: auth_user.user_id,
        })
        .await?;

    Ok(Json(FetchMeResponseBody::from(output)))
}

#[tracing::instrument(name = "handle_debug", skip())]
pub(crate) async fn handle_debug() -> impl IntoResponse {
    tracing::info!("info");
    tracing::error!("error");
    tracing::warn!("warn");
    tracing::debug!("debug");
    tracing::trace!("trace");
    (StatusCode::OK).into_response()
}
