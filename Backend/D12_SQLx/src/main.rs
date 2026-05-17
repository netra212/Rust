#![forbid(clippy::unwrap_used)]
// Prevents the use of `.unwrap()` anywhere in the project.
// This helps avoid unexpected panics and encourages proper error handling.

use axum::Router;
// Imports Axum's `Router` type used to define application routes.

mod app_state;
// Declares the `app_state` module.
// Usually contains shared application state such as DB pools/configs.

mod domain;
// Declares the `domain` module.
// Typically contains business logic grouped by feature/domain.

mod utils;
// Declares the `utils` module.
// Usually contains helper utilities and reusable functions.

use crate::app_state::AppState;
// Imports the `AppState` struct from the local crate.
// This shared state is passed into routes/handlers.

use crate::domain::todo::routes::todos_routes;
// Imports the Todo routes function.
// This function returns all routes related to todos.

use crate::utils::setup_tracing::setup_tracing;
// Imports the tracing setup function.
// Used to configure structured logging.

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

const PORT_8000: &str = "127.0.0.1:8000";
// Defines the server address and port the app will listen on.

#[tokio::main]
// Marks this as the async Tokio runtime entry point.
// Tokio starts the async executor automatically.

async fn main() {
    // Main async function where the server boots up.

    dotenvy::dotenv().ok();
    // Loads environment variables from a `.env` file if it exists.
    // `.ok()` ignores the error if no `.env` file is found.

    setup_tracing();
    // Initializes tracing/logging configuration.

    let app_state = AppState::init()
        .await
        .expect("Failed to initialize database");
    // Initializes shared application state asynchronously.
    // Usually connects to the database or loads configs.
    // Crashes the app if initialization fails.

    let app = Router::new().merge(todos_routes()).with_state(app_state);
    // Creates a new Axum router.
    // Merges Todo-related routes into the app.
    // Attaches shared application state to all handlers.

    let listener = tokio::net::TcpListener::bind(PORT_8000)
        .await
        .expect("Failed to bind to address");
    // Binds a TCP listener to `127.0.0.1:8000`.
    // The server will listen for incoming HTTP connections here.
    // Crashes if the port is already in use or binding fails.

    tracing::debug!(
        "listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );
    // Logs the address the server is listening on.
    // Useful for debugging and startup confirmation.

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
    // Starts the Axum HTTP server.
    // Continuously serves incoming requests asynchronously.
    // Crashes if the server unexpectedly fails.
}
