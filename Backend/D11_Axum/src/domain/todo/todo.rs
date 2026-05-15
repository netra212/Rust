use axum::{
    Json,                   // JSON response/request wrapper
    extract::{Path, State}, // Extract URL parameters and app state
    http::StatusCode,       // HTTP status codes (200, 404, etc.)
    response::IntoResponse, // Trait for converting types to HTTP responses
};

use serde::Serialize; // Auto-convert structs to JSON

use crate::{
    app_state::AppState, // Shared application state (thread-safe)
    domain::todo::{dto::CreateTodo, error::TodoApiError}, // DTOs and error types
};

#[derive(Debug, Serialize, Clone)] // Debug printing, JSON conversion, cloning ability
pub struct Todo {
    pub id: i32,      // Unique identifier (like a database primary key)
    pub text: String, // The actual todo content
}

// Defines methods associated with Todo.
impl Todo {
    fn handle_todo_not_found(id: i32) -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            Json(TodoApiError {
                error: format!("Todo with id {} not found", id),
            }),
        )
    }

    // Get/todos
    pub async fn get_all(State(app): State<AppState>) -> impl IntoResponse {
        let todos = app.read().unwrap();
        let todos = todos.values().cloned().collect::<Vec<_>>();

        Json(todos)
    }

    // POST / todos
    pub async fn create(
        State(app): State<AppState>,
        Json(input): Json<CreateTodo>,
    ) -> impl IntoResponse {
        let mut todos = app.write().unwrap();
        let id = todos.len() as i32 + 1;

        let todo = Todo {
            id,
            text: input.text,
        };

        todos.insert(todo.id, todo.clone());

        (StatusCode::CREATED, Json(todo))
    }

    // Get / todos {id}
    pub async fn get_by_id(Path(id): Path<i32>, State(app): State<AppState>) -> impl IntoResponse {
        let todos = app.read().unwrap();

        match todos.get(&id) {
            Some(todo) => Json(todo.clone()).into_response(),
            None => Self::handle_todo_not_found(id).into_response(),
        }
    }

    // DELETE /todos/{id}
    pub async fn delete_by_id(
        Path(id): Path<i32>,
        State(app): State<AppState>,
    ) -> impl IntoResponse {
        if app.write().unwrap().remove(&id).is_some() {
            StatusCode::NO_CONTENT.into_response()
        } else {
            Self::handle_todo_not_found(id).into_response()
        }
    }
}
