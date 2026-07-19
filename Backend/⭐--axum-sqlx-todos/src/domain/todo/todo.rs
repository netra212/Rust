use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::app_state::AppState;
use crate::domain::todo::dto::{CreateTodoDto, DeleteTodoResult, UpdateTodoDto};
use crate::domain::todo::error::TodoError;

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl Todo {
    pub async fn get_all(app_state: State<AppState>) -> Result<Json<Vec<Todo>>, TodoError> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
                SELECT * 
                FROM todos 
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(&app_state.pool)
        .await?;

        Ok(Json(todos))
    }

    pub async fn get_by_id(Path(id): Path<i32>, app_state: State<AppState>) -> Result<Json<Todo>, TodoError> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
                SELECT * 
                FROM todos 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => TodoError::NotFound(id),
            _ => TodoError::Database(e),
        })?;

        Ok(Json(todo))
    }

    pub async fn create(
        app_state: State<AppState>,
        Json(body): Json<CreateTodoDto>,
    ) -> Result<(StatusCode, Json<Todo>), TodoError> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
                INSERT INTO todos (text) 
                VALUES ($1) 
                RETURNING *
            "#,
            body.text
        )
        .fetch_one(&app_state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(todo)))
    }

    pub async fn update(
        Path(id): Path<i32>,
        app_state: State<AppState>,
        Json(body): Json<UpdateTodoDto>,
    ) -> Result<Json<Todo>, TodoError> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
                UPDATE todos
                SET text = COALESCE($1, text),
                    updated_at = NOW()
                WHERE id = $2
                RETURNING *
            "#,
            body.text,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => TodoError::NotFound(id),
            _ => TodoError::Database(e),
        })?;

        Ok(Json(todo))
    }

    pub async fn delete(
        Path(id): Path<i32>,
        app_state: State<AppState>,
    ) -> Result<Json<DeleteTodoResult>, TodoError> {
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
            sqlx::Error::RowNotFound => TodoError::NotFound(id),
            _ => TodoError::Database(e),
        })?;

        Ok(Json(result))
    }
}
