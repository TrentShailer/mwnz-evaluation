use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub error_description: String,
}

impl IntoResponse for ApiError {
    /// To match spec, all api errors are 404 errors
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NOT_FOUND, Json(self)).into_response()
    }
}
