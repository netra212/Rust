use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::app_state::AppState;
use crate::domain::author::dto::{CreateAuthorDto, DeleteAuthorResult, UpdateAuthorDto};
use crate::domain::author::error::AuthorError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Author {
    pub id: i32,
    pub name: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl Author {
    pub async fn get_all(app_state: State<AppState>) -> Result<Json<Vec<Author>>, AuthorError> {
        let authors = sqlx::query_as!(
            Author,
            r#"
                SELECT id, name, created_at, updated_at 
                FROM authors 
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(&app_state.pool)
        .await?;

        Ok(Json(authors))
    }

    pub async fn get_by_id(Path(id): Path<i32>, app_state: State<AppState>) -> Result<Json<Author>, AuthorError> {
        let author = sqlx::query_as!(
            Author,
            r#"
                SELECT id, name, created_at, updated_at 
                FROM authors 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthorError::NotFound(id),
            _ => AuthorError::Database(e),
        })?;

        Ok(Json(author))
    }

    pub async fn create(
        app_state: State<AppState>,
        Json(body): Json<CreateAuthorDto>,
    ) -> Result<(StatusCode, Json<Author>), AuthorError> {
        let author = sqlx::query_as!(
            Author,
            r#"
                INSERT INTO authors (name) 
                VALUES ($1) 
                RETURNING id, name, created_at, updated_at
            "#,
            body.name
        )
        .fetch_one(&app_state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(author)))
    }

    pub async fn update(
        Path(id): Path<i32>,
        app_state: State<AppState>,
        Json(body): Json<UpdateAuthorDto>,
    ) -> Result<Json<Author>, AuthorError> {
        let author = sqlx::query_as!(
            Author,
            r#"
                UPDATE authors
                SET name = COALESCE($1, name),
                    updated_at = NOW()
                WHERE id = $2
                RETURNING id, name, created_at, updated_at
            "#,
            body.name,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthorError::NotFound(id),
            _ => AuthorError::Database(e),
        })?;

        Ok(Json(author))
    }

    pub async fn delete(
        Path(id): Path<i32>,
        app_state: State<AppState>,
    ) -> Result<Json<DeleteAuthorResult>, AuthorError> {
        let result = sqlx::query_as!(
            DeleteAuthorResult,
            r#"
                DELETE FROM authors 
                WHERE id = $1
                RETURNING id
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthorError::NotFound(id),
            _ => AuthorError::Database(e),
        })?;

        Ok(Json(result))
    }
}
