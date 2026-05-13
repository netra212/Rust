use axum::{Router, routing::get};

use crate::{app_state::AppState, domain::todo::todo::Todo};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(Todo::get_all).post(Todo::create))
        .route("/todos/{id}", get(Todo::get_by_id))
}
