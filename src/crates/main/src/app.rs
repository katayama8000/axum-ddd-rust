use axum::{
    routing::{get, post, put},
    Router,
};

use infrastructure::{
    auth::{
        argon2_password_hasher::Argon2PasswordHasher, jwt_access_token_issuer::JwtAccessTokenIssuer,
    },
    mysql::{
        circle_duplicate_checker::CircleDuplicateChecker, circle_repository::CircleRepository,
        user_repository::UserRepository,
    },
};

use crate::{
    config::{auth::AuthConfig, connect},
    handler::{
        handle_create_circle, handle_debug, handle_fetch_all, handle_fetch_circle, handle_fetch_me,
        handle_get_version, handle_sign_in, handle_sign_up, handle_update_circle,
    },
};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) circle_repository: CircleRepository,
    pub(crate) circle_duplicate_checker: CircleDuplicateChecker,
    pub(crate) user_repository: UserRepository,
    pub(crate) password_hasher: Argon2PasswordHasher,
    pub(crate) access_token_issuer: JwtAccessTokenIssuer,
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/version", get(handle_get_version))
        .route("/circle/{id}", get(handle_fetch_circle))
        .route("/circle", get(handle_fetch_all))
        .route("/circle", post(handle_create_circle))
        .route("/circle/{id}", put(handle_update_circle))
        .route("/debug", get(handle_debug))
        .route("/auth/sign-up", post(handle_sign_up))
        .route("/auth/sign-in", post(handle_sign_in))
        // Protected: `handle_fetch_me` takes `AuthUser`, so a missing or bad token is
        // rejected with 401 before the handler runs.
        .route("/me", get(handle_fetch_me))
}

pub async fn run() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let auth_config = AuthConfig::from_env().expect("auth config should be valid");
    let pool = connect::connect().await.expect("database should connect");
    let state = AppState {
        circle_repository: CircleRepository::new(pool.clone()),
        circle_duplicate_checker: CircleDuplicateChecker::new(pool.clone()),
        user_repository: UserRepository::new(pool.clone()),
        password_hasher: Argon2PasswordHasher::new(),
        access_token_issuer: JwtAccessTokenIssuer::new(
            &auth_config.jwt_secret,
            auth_config.access_token_ttl_seconds,
        ),
    };

    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::{header::CONTENT_TYPE, StatusCode};
    use domain::{
        aggregate::{
            circle::Circle,
            member::Member,
            value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
        },
        interface::{
            access_token_issuer_interface::AccessTokenIssuerInterface,
            circle_repository_interface::CircleRepositoryInterface,
        },
    };
    use std::str::FromStr;
    use tower::ServiceExt;

    use crate::handler::{
        CreateCircleRequestBody, CreateCircleResponseBody, UpdateCircleRequestBody,
    };

    use super::*;

    const TEST_JWT_SECRET: &str = "test-secret-long-enough-for-hs256-ok";

    fn test_state(pool: &sqlx::MySqlPool) -> AppState {
        AppState {
            circle_repository: CircleRepository::new(pool.clone()),
            circle_duplicate_checker: CircleDuplicateChecker::new(pool.clone()),
            user_repository: UserRepository::new(pool.clone()),
            password_hasher: Argon2PasswordHasher::new(),
            access_token_issuer: JwtAccessTokenIssuer::new(TEST_JWT_SECRET, 3600),
        }
    }

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_version() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let state = test_state(&pool);
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/version")
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, "0.1.1");
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_circle() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let state = test_state(&pool);
        let app = router().with_state(state.clone());
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/circle")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &CreateCircleRequestBody {
                            circle_name: "circle_name1".to_string(),
                            capacity: 10,
                            owner_name: "owner1".to_string(),
                            owner_age: 21,
                            owner_grade: 3,
                            owner_major: "Music".to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = serde_json::from_slice::<'_, CreateCircleResponseBody>(
            &axum::body::to_bytes(response.into_body(), usize::MAX).await?,
        )?;

        let created = state
            .circle_repository
            .find_by_id(&CircleId::from_str(&response_body.circle_id)?)
            .await?;
        let circle = Circle::reconstruct(
            CircleId::from_str(&response_body.circle_id)?,
            "circle_name1".to_string(),
            Member::reconstruct(
                MemberId::from_str(&response_body.owner_id)?,
                "owner1".to_string(),
                21,
                Grade::try_from(3)?,
                Major::Music,
            ),
            10,
            vec![],
        );
        assert_eq!(created, circle);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_fetch_circle() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let state = test_state(&pool);
        let app = router().with_state(state);
        let unexist_circle_id = 0;
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri(format!("/circle/{}", unexist_circle_id))
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, "Circle not found");

        let (circle_id, owner_id) = build_circle(&app).await?;

        let fetched_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri(format!("/circle/{}", circle_id))
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(fetched_response.status(), StatusCode::OK);
        let fetched_response_body = String::from_utf8(
            axum::body::to_bytes(fetched_response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(
            fetched_response_body,
            format!(
                "{{\"circle_id\":{},\"circle_name\":\"Music club\",\"capacity\":10,\"owner\":{{\"id\":{},\"name\":\"John Lennon\",\"age\":21,\"grade\":3,\"major\":\"Music\"}},\"members\":[]}}",
                circle_id,owner_id
            )
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_circle() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let state = test_state(&pool);
        let app = router().with_state(state.clone());
        let (circle_id, _) = build_circle(&app).await?;
        let update_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PUT")
                    .uri(format!("/circle/{}", circle_id))
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &UpdateCircleRequestBody {
                            circle_name: Some("Football club".to_string()),
                            capacity: Some(20),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(update_response.status(), StatusCode::OK);

        let updated_circle = state
            .circle_repository
            .find_by_id(&CircleId::from_str(&circle_id)?)
            .await?;
        assert_eq!(updated_circle.name, "Football club");
        assert_eq!(updated_circle.capacity, 20);

        Ok(())
    }

    async fn build_circle(app: &Router) -> anyhow::Result<(String, String)> {
        let create_response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/circle")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &CreateCircleRequestBody {
                            circle_name: "Music club".to_string(),
                            capacity: 10,
                            owner_name: "John Lennon".to_string(),
                            owner_age: 21,
                            owner_grade: 3,
                            owner_major: "Music".to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(create_response.status(), StatusCode::OK);
        let create_response_body = serde_json::from_slice::<CreateCircleResponseBody>(
            &axum::body::to_bytes(create_response.into_body(), usize::MAX).await?,
        )?;

        Ok((
            create_response_body.circle_id,
            create_response_body.owner_id,
        ))
    }

    /// A pool that never dials the database. Enough for the paths that are rejected before any
    /// repository is touched, which is exactly what the auth guard is supposed to do.
    fn lazy_state() -> AppState {
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .connect_lazy("mysql://user:password@127.0.0.1:3306/unused")
            .expect("a lazy pool should be constructible");
        test_state(&pool)
    }

    async fn get_me(authorization: Option<&str>) -> anyhow::Result<StatusCode> {
        let app = router().with_state(lazy_state());
        let mut request = axum::http::Request::builder().method("GET").uri("/me");
        if let Some(value) = authorization {
            request = request.header(axum::http::header::AUTHORIZATION, value);
        }
        let response = app
            .oneshot(request.body(axum::body::Body::empty())?)
            .await?;
        Ok(response.status())
    }

    #[tokio::test]
    async fn test_me_without_a_token_is_401() -> anyhow::Result<()> {
        assert_eq!(get_me(None).await?, StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    async fn test_me_with_a_malformed_header_is_401() -> anyhow::Result<()> {
        for header in [
            "",
            "abc.def.ghi",
            "Basic dXNlcjpwYXNz",
            "Bearer ",
            "Bearer x",
        ] {
            assert_eq!(
                get_me(Some(header)).await?,
                StatusCode::UNAUTHORIZED,
                "{header:?} should be rejected"
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_me_with_a_token_signed_by_another_secret_is_401() -> anyhow::Result<()> {
        let forged = JwtAccessTokenIssuer::new("an-attacker-controlled-secret-value!!", 3600)
            .issue(&domain::aggregate::value_object::user_id::UserId::gen())?;

        assert_eq!(
            get_me(Some(&format!("Bearer {}", forged.value))).await?,
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_up_with_an_invalid_email_is_400() -> anyhow::Result<()> {
        // Input is validated in the use case before the repository is reached, so this needs
        // no database either.
        let app = router().with_state(lazy_state());
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-up")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &crate::handler::SignUpRequestBody {
                            email: "not-an-email".to_string(),
                            password: "correct horse battery staple".to_string(),
                        },
                    )?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_up_with_a_short_password_is_400() -> anyhow::Result<()> {
        let app = router().with_state(lazy_state());
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-up")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &crate::handler::SignUpRequestBody {
                            email: "user@example.com".to_string(),
                            password: "short".to_string(),
                        },
                    )?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_sign_up_then_sign_in_then_fetch_me() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let app = router().with_state(test_state(&pool));

        let email = format!("{}@example.com", CircleId::gen());
        let password = "correct horse battery staple".to_string();

        let sign_up_response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-up")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &crate::handler::SignUpRequestBody {
                            email: email.clone(),
                            password: password.clone(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(sign_up_response.status(), StatusCode::CREATED);

        let sign_in_response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-in")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &crate::handler::SignInRequestBody {
                            email: email.clone(),
                            password,
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(sign_in_response.status(), StatusCode::OK);
        let sign_in_body = serde_json::from_slice::<crate::handler::SignInResponseBody>(
            &axum::body::to_bytes(sign_in_response.into_body(), usize::MAX).await?,
        )?;

        let me_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/me")
                    .header(
                        axum::http::header::AUTHORIZATION,
                        format!("Bearer {}", sign_in_body.access_token),
                    )
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(me_response.status(), StatusCode::OK);
        let me_body = serde_json::from_slice::<crate::handler::FetchMeResponseBody>(
            &axum::body::to_bytes(me_response.into_body(), usize::MAX).await?,
        )?;

        assert_eq!(me_body.email, email.to_lowercase());
        assert_eq!(me_body.user_id, sign_in_body.user_id);
        Ok(())
    }

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_sign_in_with_a_wrong_password_is_401() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let app = router().with_state(test_state(&pool));

        let email = format!("{}@example.com", CircleId::gen());
        let sign_up_response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-up")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &crate::handler::SignUpRequestBody {
                            email: email.clone(),
                            password: "correct horse battery staple".to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(sign_up_response.status(), StatusCode::CREATED);

        let sign_in_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-in")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &crate::handler::SignInRequestBody {
                            email,
                            password: "incorrect horse battery staple".to_string(),
                        },
                    )?))?,
            )
            .await?;

        assert_eq!(sign_in_response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_sign_up_with_a_duplicate_email_is_409() -> anyhow::Result<()> {
        let pool = connect::connect_test()
            .await
            .expect("database should connect");
        let app = router().with_state(test_state(&pool));

        let email = format!("{}@example.com", CircleId::gen());
        let body = || -> anyhow::Result<axum::body::Body> {
            Ok(axum::body::Body::new(serde_json::to_string(
                &crate::handler::SignUpRequestBody {
                    email: email.clone(),
                    password: "correct horse battery staple".to_string(),
                },
            )?))
        };

        let first = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-up")
                    .header(CONTENT_TYPE, "application/json")
                    .body(body()?)?,
            )
            .await?;
        assert_eq!(first.status(), StatusCode::CREATED);

        let second = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/auth/sign-up")
                    .header(CONTENT_TYPE, "application/json")
                    .body(body()?)?,
            )
            .await?;

        assert_eq!(second.status(), StatusCode::CONFLICT);
        Ok(())
    }
}
