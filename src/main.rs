use crate::handler::{handle_create_circle, handle_fetch_circle, handle_get, handle_update_circle};

mod domain;
mod handler;
mod infrastructure;
mod usecase;

use axum::{
    routing::{get, post, put},
    Router,
};

#[derive(Clone)]
struct AppState {
    counter: usize,
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get))
        .route("/circle/:id", get(handle_fetch_circle))
        .route("/circle", post(handle_create_circle))
        .route("/circle/:id", put(handle_update_circle))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let state = AppState { counter: 0 };

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
    use axum::http::StatusCode;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn test_create_circle() -> anyhow::Result<()> {
        let state = AppState { counter: 0 };
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    // FIXME: why don't you use request body
                    .uri("/circle?circle_name=circle_name1&capacity=1&owner_name=owner1&owner_age=21&owner_grade=3&owner_major=Music")
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        // FIXME: I think CircleRepository::create is incorrect.
        assert_eq!(response_body, "Circle already exists");
        // FIXME: check state
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_circle() -> anyhow::Result<()> {
        // FIXME: prepare state
        let state = AppState { counter: 0 };
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/circle/1")
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, ""); // FIXME
        Ok(())
    }

    #[tokio::test]
    async fn test_root() -> anyhow::Result<()> {
        let state = AppState { counter: 0 };
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
        assert_eq!(response_body, "Hello, World!");
        Ok(())
    }

    #[tokio::test]
    async fn test_update_circle() -> anyhow::Result<()> {
        // FIXME: prepare state
        let state = AppState { counter: 0 };
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
