use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::app_state::AppState;
use crate::domain::todo::{CreateTodo, TodoAPiError};
use serde::{Deserialize, Serialize};

use axum::IntoResponse;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub text: String,
}

impl Todo {
    // Post -> todos because I am sending data.
    pub async fn CreateTodo(
        State(app): State<AppState>,
        State(input): Json<CreateTodo>,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let mut todos = app.write().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Lock poisoned".to_string(),
            )
        })?;

        /**
        * map_err() -> is a method on the Result which transforms the error value without changing the success value.
        result.map_err(|error_variable| new_error_value)
                     //                  ^^^^^^^^^^^     ^^^^^^^^^^^
                     //                  Original error  What you want instead
        Underscore (_) -> means I don't care about this value, ignore it.

        ----------------------------------------------------------------
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock poisoned"))
                               ^
        Ignoring the actual PoisonError because we just want to return a generic error message to the client.

        ----------------------------------------------------------------
        # When to use |error| or |_| ?
        1.Use `|_|` when you DON'T need the original error
        CODE:
            .map_err(|_| "Something failed")  // Generic message

        2.Use `|error|` when you NEED the original error
        CODE:
        .map_err(|error| {eprintln!("Actual error: {}", error);})


        */
        let ids = todos.id;

        let todo = Todo {
            ids,
            text: input.text,
        };

        // Inserting the data.
        todo.insert(todo.id, todo.clone()); // Without clone(): todo would move to HashMap directly, becoming unusable afterward. Inserting a copy.

        Ok((StatusCode::CREATED, Json(todo)));
    }

    // Get /todos.
    pub async fn getAllTodos(State(app): State<AppState>) -> IntoResponse {
        let todos = app.read().unwrap();
        ()
    }
}


/**
 * The Mental Model
    Think of Result<T, E> as a box with two compartments:
        ┌─────────────────┐
        │  Result<T, E>   │
        ├────────┬────────┤
        │ Success│ Error  │
        │ (T)    │ (E)    │
        └────────┴────────┘

        map_err() transforms ONLY the Error compartment:
        ┌─────────────────┐        ┌──────────────────┐
        │  Result<T, E1>  │        │  Result<T, E2>   │
        ├────────┬────────┤   →    ├────────┬─────────┤
        │ Success│ Error  │        │ Success│ Error   │
        │ (T)    │ (E1)   │        │ (T)    │ (E2)    │
        └────────┴────────┘        └────────┴─────────┘
                 │                           │
                 └── map_err(|e1| e2) ───────┘

                 Key Takeaways
map_err() changes error type, not success type

|_| ignores the original error (use when you don't need it)

|e| uses the original error (use for logging/debugging)

Always combine with ? for clean error propagation

Never use unwrap() in web servers - it crashes the entire server!

The pattern app.write().map_err(|_| error_response)? is considered idiomatic Rust for web handlers because it's:
 */