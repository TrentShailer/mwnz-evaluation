use axum::{
    async_trait,
    extract::{path::ErrorKind, rejection::PathRejection, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub error_description: String,
}

/// from https://github.com/tokio-rs/axum/blob/main/examples/customize-path-rejection/src/main.rs<br>
/// Gives Path extractor ApiError body
pub struct PathExctractor<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for PathExctractor<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<ApiError>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let status = StatusCode::BAD_REQUEST;

                        let kind = inner.into_kind();

                        let body = match &kind {
                            ErrorKind::Message(msg) => ApiError {
                                error: "Bad Request".to_string(),
                                error_description: msg.clone(),
                            },

                            _ => ApiError {
                                error: "Bad Request".to_string(),
                                error_description: kind.to_string(),
                            },
                        };

                        (status, body)
                    }
                    PathRejection::MissingPathParams(error) => (
                        StatusCode::BAD_REQUEST,
                        ApiError {
                            error: "Bad Request".to_string(),
                            error_description: error.to_string(),
                        },
                    ),
                    _ => (
                        StatusCode::BAD_REQUEST,
                        ApiError {
                            error: "Bad Request".to_string(),
                            error_description: String::new(),
                        },
                    ),
                };

                Err((status, axum::Json(body)))
            }
        }
    }
}
