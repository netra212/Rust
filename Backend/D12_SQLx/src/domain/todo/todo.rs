// Import Json extractor/response type from Axum.
// Used to send/receive JSON in HTTP requests/responses.
use axum::Json;

// Import extractors:
// - Path: gets values from URL like /todo/1
// - State: gives access to shared app state (DB pool etc.)
use axum::extract::{Path, State};

// Import HTTP status codes like 200, 201, 404, etc.
use axum::http::StatusCode;

// Import Serialize trait so structs can be converted into JSON responses.
use serde::Serialize;

// Import FromRow so SQLx can map database rows into Rust structs.
use sqlx::FromRow;

// Import OffsetDateTime (represents timestamps from DB)
// NOTE: used for created_at / updated_at fields.
use time::OffsetDateTime;

// Import shared application state (contains DB pool).
use crate::app_state::AppState;

// Import DTOs (Data Transfer Objects):
// - CreateTodoDto: data for creating todo
// - UpdateTodoDto: data for updating todo
// - DeleteTodoResult: response after delete
use crate::domain::todo::dto::{CreateTodoDto, DeleteTodoResult, UpdateTodoDto};

// Import custom application error type.
use crate::domain::todo::error::TodoError;

/// Represents a Todo item stored in the database.
/// This struct maps directly to the `todos` table.
#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    /// Unique ID of the todo (primary key in DB).
    pub id: i32,

    /// Actual text/content of the todo item.
    pub text: String,

    /// Timestamp when the todo was created.
    /// Serialized into ISO8601 format when sent as JSON.
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,

    /// Timestamp when the todo was last updated.
    /// Also serialized into ISO8601 format.
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

impl Todo {
    /// Fetch all todos from the database.
    /// Returns a JSON list of Todo items.
    pub async fn get_all(
        app_state: State<AppState>, // Extract shared app state (DB pool)
    ) -> Result<Json<Vec<Todo>>, TodoError> {
        // Run SQL query and map rows into Vec<Todo>
        let todos = sqlx::query_as!(
            Todo, // map result into Todo struct
            r#"
                SELECT *
                FROM todos
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(&app_state.pool) // execute query using DB pool
        .await?; // if error happens, convert into TodoError

        // Wrap result in JSON response
        Ok(Json(todos))
    }

    /// Fetch a single todo by its ID from the URL path.
    pub async fn get_by_id(
        Path(id): Path<i32>,        // extract `id` from URL like /todos/1
        app_state: State<AppState>, // get DB pool
    ) -> Result<Json<Todo>, TodoError> {
        // Query database for a single todo
        let todo = sqlx::query_as!(
            Todo,
            r#"
                SELECT *
                FROM todos
                WHERE id = $1
            "#,
            id // bind parameter $1 = id
        )
        .fetch_one(&app_state.pool) // expect exactly one row
        .await
        .map_err(|e| match e {
            // If no row found, return custom NotFound error
            sqlx::Error::RowNotFound => TodoError::NotFound(id),

            // Any other DB error becomes generic database error
            _ => TodoError::Database(e),
        })?;

        Ok(Json(todo))
    }

    /// Create a new todo in the database.
    pub async fn create(
        app_state: State<AppState>,      // DB access
        Json(body): Json<CreateTodoDto>, // parse JSON request body
    ) -> Result<(StatusCode, Json<Todo>), TodoError> {
        // Insert new todo into DB and return created row
        let todo = sqlx::query_as!(
            Todo,
            r#"
                INSERT INTO todos(text)
                VALUES ($1)
                RETURNING *
            "#,
            body.text // value from request body
        )
        .fetch_one(&app_state.pool)
        .await?;

        // Return 201 CREATED + created todo as JSON
        Ok((StatusCode::CREATED, Json(todo)))
    }

    /// Update an existing todo by ID.
    pub async fn update(
        Path(id): Path<i32>,             // todo ID from URL
        app_state: State<AppState>,      // DB pool
        Json(body): Json<UpdateTodoDto>, // request body
    ) -> Result<Json<Todo>, TodoError> {
        // Update todo in DB
        let todo = sqlx::query_as!(
            Todo,
            r#"
                UPDATE todos
                SET text = COALESCE($1, text), // if new text is None, keep old value
                updated_at = NOW()             // update timestamp
                WHERE id = $2
                RETURNING *
            "#,
            body.text, // $1 = optional new text
            id         // $2 = todo id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            // If no row updated → todo not found
            sqlx::Error::RowNotFound => TodoError::NotFound(id),

            // Any other DB error
            _ => TodoError::Database(e),
        })?;

        Ok(Json(todo))
    }

    /// Delete a todo by ID.
    pub async fn delete(
        Path(id): Path<i32>,        // extract todo id from URL
        app_state: State<AppState>, // DB pool
    ) -> Result<Json<DeleteTodoResult>, TodoError> {
        // Delete todo and return deleted id
        let result = sqlx::query_as!(
            DeleteTodoResult,
            r#"
                DELETE FROM todos
                WHERE id = $1
                RETURNING id
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            // If nothing deleted → not found
            sqlx::Error::RowNotFound => TodoError::NotFound(id),

            // Other DB errors
            _ => TodoError::Database(e),
        })?;

        Ok(Json(result))
    }
}
