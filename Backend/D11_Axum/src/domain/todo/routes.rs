use axum::{Router, routing::get};

use crate::{app_state::AppState, domain::todo::todo::Todo};

// This below function return the Router with AppState.
pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(Todo::get_all).post(Todo::create))
        .route(
            "/todos/{id}",
            get(Todo::get_by_id).delete(Todo::delete_by_id),
        )
}

// Router::new()
// .route("/todos", get(Todo::get_all).post(Todo::create))
// .route("/todos/{id}", get(Todo::get_by_id).delete(Todo::delete_by_id))
