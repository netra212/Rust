// Import Router type from Axum.
// Router is used to define all HTTP routes in the application.
use axum::Router;

// Import HTTP method helpers (GET, POST, etc.)
// These define what kind of request each route accepts.
use axum::routing::get;

// Import shared application state (contains DB pool, config, etc.)
use crate::app_state::AppState;

// Import Todo handlers (get_all, create, update, delete, etc.)
use crate::domain::todo::todo::Todo;

/// Builds and returns all routes related to Todo feature.
///
/// Router<AppState> means:
/// - This router has access to shared application state
/// - AppState will be available in every handler via `State<AppState>`
pub fn todos_routes() -> Router<AppState> {
    Router::new() // Create a new empty router
        // Define routes for "/todos"
        //
        // GET  /todos    → fetch all todos
        // POST /todos    → create a new todo
        .route(
            "/todos", // URL path (collection endpoint)
            get(Todo::get_all) // handler for GET request
                .post(Todo::create), // handler for POST request
        )
        // Define routes for "/todos/{id}"
        //
        // This is a "single resource" route (one todo by ID)
        //
        // GET    /todos/{id} → get todo by ID
        // PUT    /todos/{id} → update todo
        // DELETE /todos/{id} → delete todo
        .route(
            "/todos/{id}",
            get(Todo::get_by_id) // fetch a single todo by ID
                // NOTE:
                // Axum uses `.put()` for updates, not `.path()`
                // Your original `.path()` is incorrect
                .put(Todo::update) // update existing todo
                .delete(Todo::delete), // delete todo by ID
        )
}
