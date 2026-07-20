use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatsApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum StatsError {
    #[error("Author with id {0} not found")]
    NotFound(i32),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for StatsError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            StatsError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            StatsError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        (status_code, Json(StatsApiError { error: error_message })).into_response()
    }
}
