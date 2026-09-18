use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use usecase::{fetch_me::FetchMeError, sign_in::SignInError, sign_up::SignUpError};

/// The HTTP-facing error type.
///
/// Handlers used to return `Result<_, String>`, which axum renders as `200 OK` with the message
/// in the body — so a failure looked like a success and 401 could not be expressed at all.
#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorResponseBody {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message),
            AppError::Internal(error) => {
                // The cause is for us, not for the client: it can carry SQL, connection
                // strings and other details that should not leave the server.
                tracing::error!("internal error: {:?}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(ErrorResponseBody { message })).into_response()
    }
}

impl From<SignUpError> for AppError {
    fn from(error: SignUpError) -> Self {
        match error {
            SignUpError::EmailAlreadyTaken => AppError::Conflict(error.to_string()),
            SignUpError::InvalidInput(cause) => AppError::BadRequest(cause.to_string()),
            SignUpError::Other(cause) => AppError::Internal(cause),
        }
    }
}

impl From<SignInError> for AppError {
    fn from(error: SignInError) -> Self {
        match error {
            // Same response for an unknown email and a wrong password.
            SignInError::InvalidCredentials => AppError::Unauthorized,
            SignInError::Other(cause) => AppError::Internal(cause),
        }
    }
}

impl From<FetchMeError> for AppError {
    fn from(error: FetchMeError) -> Self {
        match error {
            FetchMeError::UserNotFound => AppError::NotFound(error.to_string()),
            FetchMeError::Other(cause) => AppError::Internal(cause),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn status_and_body(error: AppError) -> (StatusCode, String) {
        let response = error.into_response();
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn test_unauthorized_is_401() {
        let (status, body) = status_and_body(AppError::Unauthorized).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body, r#"{"message":"Unauthorized"}"#);
    }

    #[tokio::test]
    async fn test_conflict_is_409() {
        let (status, body) = status_and_body(AppError::Conflict(
            "Email is already registered".to_string(),
        ))
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(body, r#"{"message":"Email is already registered"}"#);
    }

    #[tokio::test]
    async fn test_internal_does_not_leak_the_cause() {
        let (status, body) = status_and_body(AppError::Internal(anyhow::Error::msg(
            "connection refused to mysql://user:hunter2@db",
        )))
        .await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body, r#"{"message":"Internal server error"}"#);
    }

    #[tokio::test]
    async fn test_sign_in_failure_maps_to_401() {
        let (status, _) = status_and_body(SignInError::InvalidCredentials.into()).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_sign_up_duplicate_maps_to_409() {
        let (status, _) = status_and_body(SignUpError::EmailAlreadyTaken.into()).await;
        assert_eq!(status, StatusCode::CONFLICT);
    }
}
