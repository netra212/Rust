use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BlogApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BlogError {
    #[error("Blog with id {0} not found")]
    NotFound(i32),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            BlogError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BlogError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        (status_code, Json(BlogApiError { error: error_message })).into_response()
    }
}
