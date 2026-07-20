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
        _app_state: State<AppState>,
        _body: Json<CreateAuthorDto>,
    ) -> Result<(StatusCode, Json<Author>), AuthorError> {
        // TODO: Insert a new author and return it with 201 Created
        let author = sqlx::query_as!(
            Author,
            r#"
                INSERT INTO authors (name)
                VALUES ($1)
                RETURNING id, name, created_at, updated_at
            "#,
            _body.name
        )
        .fetch_one(&_app_state.pool)
        .await?;

        Ok(StatusCode::CREATED, Json(author))
    }

    pub async fn update(
        _path: Path<i32>,
        _app_state: State<AppState>,
        _body: Json<UpdateAuthorDto>,
    ) -> Result<Json<Author>, AuthorError> {
        // TODO: Partially update an author's fields and updated_at, returning NotFound if missing
        let id = path.into_inner();
        let author = sqlx::query_as!(
            Author,
            r#"
                UPDATE authors
                SET name = COALESCE($1, name), 
                    updated_at = NOW()
                WHERE id = $2
                RETURNING id, name, created_at, updated_at
            "#
            body.name,
            id
        )
        .fetch_one(&_app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthorError::NOT_FOUND(id),
            _ => AuthorError::Database(e),
        })?;

        Ok(Json(author))
    }

    pub async fn delete(
        _path: Path<i32>,
        _app_state: State<AppState>,
    ) -> Result<Json<DeleteAuthorResult>, AuthorError> {
        // TODO: Delete an author by id and return the deleted id, or NotFound if missing
        let id = _path.into_inner();
        let author = sqlx::query_as!(
            DeleteAuthorResult,
            r#"
                DELETE FROM authors
                WHERE id = $1
                RETURNING id
            "#,
            id
        )
        .fetch_one(&_app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthorError::NotFound(id),
            _ => AuthorError::Database(e),
        })?;

        Ok(Json(author))
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// create:
//   - "INSERT INTO authors (name) VALUES ($1) RETURNING id, name, created_at, updated_at"
//   - fetch_one + return (StatusCode::CREATED, Json(author))
//   - Look up: RETURNING, fetch_one, StatusCode::CREATED
//
// update:
//   - "UPDATE authors SET name = COALESCE($1, name), updated_at = NOW() WHERE id = $2 RETURNING id, name, created_at, updated_at"
//   - fetch_optional + ok_or(AuthorError::NotFound(id))
//   - Look up: COALESCE, NOW(), fetch_optional
//
// delete:
//   - "DELETE FROM authors WHERE id = $1 RETURNING id"
//   - sqlx::query_as!(DeleteAuthorResult, ...).fetch_optional(...).await?
//   - Look up: DELETE ... RETURNING, fetch_optional, ok_or
