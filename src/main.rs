use crate::handler::{handle_create_circle, handle_fetch_circle, handle_update_circle};

mod domain;
mod handler;
mod infrastructure;
mod usecase;

use axum::{
    routing::{get, post, put},
    Router,
};
use handler::handle_get_version;
use infrastructure::circle_repository::CircleRepository;

#[derive(Clone)]
struct AppState {
    circle_repository: CircleRepository,
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_version))
        .route("/circle/:id", get(handle_fetch_circle))
        .route("/circle", post(handle_create_circle))
        .route("/circle/:id", put(handle_update_circle))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let state = AppState {
        circle_repository: CircleRepository::new(),
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
    use tower::ServiceExt;

    use crate::{
        domain::{
            aggregate::{
                circle::Circle,
                member::Member,
                value_object::{
                    circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId,
                },
            },
            port::circle_repository_port::CircleRepositoryPort as _,
        },
        handler::{CreateCircleRequestBody, CreateCircleResponseBody},
    };

    use super::*;

    #[tokio::test]
    async fn test_version() -> anyhow::Result<()> {
        let state = AppState {
            circle_repository: CircleRepository::new(),
        };
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, "0.1.0");
        Ok(())
    }

    #[tokio::test]
    async fn test_create_circle() -> anyhow::Result<()> {
        let state = AppState {
            circle_repository: CircleRepository::new(),
        };
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
            .find_circle_by_id(&CircleId::new(response_body.circle_id))?;
        let circle = Circle::reconstruct(
            CircleId::new(response_body.circle_id),
            "circle_name1".to_string(),
            Member::reconstruct(
                MemberId::new(response_body.owner_id),
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
    async fn test_fetch_circle() -> anyhow::Result<()> {
        let state = AppState {
            circle_repository: CircleRepository::new(),
        };
        let app = router().with_state(state);
        let unexist_circle_id = 1;
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

        let response = app
            .clone()
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

        let response_body = serde_json::from_slice::<CreateCircleResponseBody>(
            &axum::body::to_bytes(response.into_body(), usize::MAX).await?,
        )?;

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri(format!("/circle/{}", response_body.circle_id))
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let fetched_response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(
            fetched_response_body,
            format!(
                "{{\"circle_id\":{},\"circle_name\":\"circle_name1\",\"capacity\":10,\"owner\":{{\"id\":{},\"name\":\"owner1\",\"age\":21,\"grade\":\"Third\",\"major\":\"Music\"}},\"members\":[]}}",
                response_body.circle_id, response_body.owner_id
            ) 
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_update_circle() -> anyhow::Result<()> {
        let state = AppState {
            circle_repository: CircleRepository::new(),
        };
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PUT")
                    // FIXME: why don't you use request body
                    .uri("/circle/1?id=2&circle_name=circle_name2&capacity=2")
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
        // FIXME: check state
        Ok(())
    }
}
