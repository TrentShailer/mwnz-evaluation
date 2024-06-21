use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::{api_error::ApiError, router, Company};

#[tokio::test]
async fn valid_id() {
    let router = router();

    let response = router
        .oneshot(
            Request::builder()
                .uri("/v1/companies/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(
        &body[..],
        &serde_json::to_vec(&Company {
            id: 1,
            name: "MWNZ".to_string(),
            description: "..is awesome".to_string()
        })
        .unwrap()
    );
}

#[tokio::test]
async fn invalid_id() {
    let router = router();

    let response = router
        .oneshot(
            Request::builder()
                .uri("/v1/companies/-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(
        &body[..],
        &serde_json::to_vec(&ApiError {
            error: "Company not found".to_string(),
            error_description: "No company with id -1 exists.".to_string()
        })
        .unwrap()
    );
}
