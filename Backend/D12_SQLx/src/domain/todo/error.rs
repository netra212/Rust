// Import Axum's Json type.
// Used to send JSON responses back to the client.
use axum::Json;

// Import HTTP status codes like 404, 500, etc.
use axum::http::StatusCode;

// Import:
// - IntoResponse -> converts a type into an HTTP response
// - Response -> actual HTTP response type
use axum::response::{IntoResponse, Response};

// Import Serialize trait from Serde.
// Needed to convert Rust structs into JSON.
use serde::Serialize;

// Import Error derive macro from thiserror crate.
// Helps create clean custom error types.
use thiserror::Error;

// Automatically derive:
// - Debug -> allows printing for debugging
// - Serialize -> allows JSON conversion
#[derive(Debug, Serialize)]
pub struct TodoApiError {
    // JSON field:
    // {
    //   "error": "message"
    // }
    pub error: String,
}

// Automatically derive:
// - Error -> implements std::error::Error
// - Debug -> debugging support
#[derive(Error, Debug)]
pub enum TodoError {
    // Error message template.
    // {0} means the first value inside NotFound(i32)
    #[error("Todo with id {0} not found")]
    // Example:
    // TodoError::NotFound(5)
    NotFound(i32),

    // Error message template for database errors.
    #[error("Database error: {0}")]
    // Stores sqlx database errors.
    //
    // #[from] automatically converts:
    // sqlx::Error -> TodoError
    //
    // This allows using ? operator easily.
    Database(#[from] sqlx::Error),
}

// Implement IntoResponse trait for TodoError.
//
// This tells Axum how to convert our custom
// error type into an HTTP response.
impl IntoResponse for TodoError {
    // Convert TodoError into HTTP Response
    fn into_response(self) -> Response {
        // Match different error variants
        // and return:
        // - status code
        // - error message
        let (status_code, error_message) = match self {
            // If todo is not found:
            // Return:
            // - 404 status code
            // - error message string
            TodoError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),

            // If database error occurs
            TodoError::Database(ref e) => {
                // Log the actual database error internally
                tracing::error!("Database error: {}", e);

                // Return generic error to client
                // for security reasons.
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        };

        // Build final HTTP response
        (
            // HTTP status code
            status_code,
            // JSON response body
            Json(TodoApiError {
                error: error_message,
            }),
        )
            // Convert tuple into Axum Response
            .into_response()
    }
}
