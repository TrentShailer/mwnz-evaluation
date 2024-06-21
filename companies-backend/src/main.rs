pub mod api_error;
pub mod companies_route;
pub mod company_type;
pub mod fetch_company;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use companies_route::get_companies;
pub use company_type::Company;
pub use fetch_company::try_fetch_company;
use simplelog::{Config, TermLogger};

#[tokio::main]
async fn main() {
    TermLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Never,
    )
    .expect("Failed to init termlogger");

    let router = router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to create TCP listener");

    log::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .expect("Failed to start webserver")
}

pub fn router() -> Router {
    Router::new().route("/v1/companies/:id", get(get_companies))
}

#[cfg(test)]
mod integration {
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
}
