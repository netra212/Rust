use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthorApiError {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthorError {
    #[error("Author with id {0} not found")]
    NotFound(i32),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AuthorError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AuthorError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AuthorError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        (status_code, Json(AuthorApiError { error: error_message })).into_response()
    }
}
