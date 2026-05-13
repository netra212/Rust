use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use serde::Serialize;

use crate::{
    app_state::AppState,
    domain::todo::{dto::CreateTodo, error::TodoApiError},
};

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub text: String,
}

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

        Json(todos);
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

        (StatusCode::CREATED, Json(todo));
    }

    // Get / todos {id}
    pub async fn get_by_id(Path(id): Path<i32>, State(app): State<AppState>) -> impl IntoResponse {
        let todos = app.read().unwrap();

        match todos.get(&id) {
            Some(todo) => Json(todo.clone()).into_response(),
            None => Self::handle_todo_not_found(id).into_response(),
        }
    }
}
