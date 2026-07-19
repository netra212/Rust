use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum TodoError {
    #[error("Todo with id {0} not found")]
    NotFound(i32),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for TodoError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            TodoError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            TodoError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status_code, Json(TodoApiError { error: error_message })).into_response()
    }
}
