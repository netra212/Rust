use axum::http::StatusCode;
// Imports HTTP status codes from Axum.
// Used to return proper HTTP responses like 404, 500, 408, etc.

use tower::BoxError;
// Imports `BoxError`, a boxed dynamic error type from Tower.
// Allows handling different error types through a single interface.

/* ========================================================== */
/*                     ✨ MIDDLEWARE ✨                         */
/* ========================================================== */

pub async fn handle_timeout_error(
    error: BoxError,
    // Receives a generic boxed error from middleware/service layers.
) -> Result<StatusCode, (StatusCode, String)> {
    // Async middleware error handler function.
    //
    // Return Types:
    // - `Ok(StatusCode)`:
    //      returns a valid HTTP status code response.
    //
    // - `Err((StatusCode, String))`:
    //      returns a custom error response with:
    //      - HTTP status code
    //      - error message string

    if error.is::<tower::timeout::error::Elapsed>() {
        // Checks if the error type is a timeout error.
        // `Elapsed` occurs when a request exceeds the allowed timeout duration.

        Ok(StatusCode::REQUEST_TIMEOUT)
        // Returns HTTP 408 Request Timeout.
    } else {
        // Handles all other unknown/internal errors.

        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            // Returns HTTP 500 Internal Server Error.
            format!("Unhandled internal error: {error}"),
            // Creates a formatted error message including the actual error.
        ))
    }
}
