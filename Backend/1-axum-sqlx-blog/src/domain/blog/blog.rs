use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::app_state::AppState;
use crate::domain::blog::dto::{CreateBlogDto, DeleteBlogResult, UpdateBlogDto};
use crate::domain::blog::error::BlogError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl Blog {
    pub async fn get_all(app_state: State<AppState>) -> Result<Json<Vec<Blog>>, BlogError> {
        let blogs = sqlx::query_as!(
            Blog,
            r#"
                SELECT * 
                FROM blogs 
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(&app_state.pool)
        .await?;

        Ok(Json(blogs))
    }

    pub async fn get_by_id(_path: Path<i32>, _app_state: State<AppState>) -> Result<Json<Blog>, BlogError> {
        // TODO: Query blog by id, returning NotFound if it doesn't exist
        let id = _path.into_inner();
        let blog = sqlx::query_as!(
            Blog, 
            r#"
                SELECT *
                FROM blogs
                WHERE id = &1
            "#, 
            id, 
        )
        .fetch_one(&_app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BlogError::NotFound(e), 
            - => BlogError::Database(e)
        })?;

        Ok(Json(blog))
    }

    pub async fn get_all_by_author_id(
        _path: Path<i32>,
        _app_state: State<AppState>,
    ) -> Result<Json<Vec<Blog>>, BlogError> {
        // TODO: Query all blogs for a given author_id, ordered by created_at DESC
        let id = _path.into_inner();
        let blogs = sqlx::query_as!(
            Blog, 
            r#"
                SELECT *
                FROM blogs
                WHERE author_id = $1
                ORDER BY created_at DESC
            "#,
            author_id
        )
        .fetch_all(&app_state.pool)
        .await?;

        Ok(Json(blogs))
    }

    pub async fn create(
        _app_state: State<AppState>,
        _body: Json<CreateBlogDto>,
    ) -> Result<(StatusCode, Json<Blog>), BlogError> {
        // TODO: Insert a new blog (title, content, author_id) and return it with 201 Created
        let blog = sqlx::query_as!(
            Blog, 
            r#"
                INSERT INTO blogs (title, content, author_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#, 
            _body.title, 
            _body.content, 
            _body.author_id
        )
        .fetch_one(&app_state.pool)
        .await?;

        Ok(StatusCode::CREATED, Json(Blog))
    }

    pub async fn update(
        _path: Path<i32>,
        _app_state: State<AppState>,
        _body: Json<UpdateBlogDto>,
    ) -> Result<Json<Blog>, BlogError> {
        // TODO: Partially update a blog's fields and updated_at, returning NotFound if missing
        let id = _path.into_inner();
        let blog = sqlx::query_as!(
            Blog, 
            r#"
                UPDATE blogs
                SET title = COALESCE($1, title), 
                    content = COALESCE($2, content), 
                    author_id = COALESCE($3, author_id), 
                    updated_at = NOW()
                WHERE id = $4
                RETURNING *
            "#,
            _body.title,
            _body.content,
            _body.author_id,
            id
        )
        .fetch_one(&_app_state.pool)
        .await
        .map_err(|e| match e{
            sqlx::Error::RowNotFound => BlogError::NotFound(id),
            _ => BlogError::Database(e),
        })?;

        Ok(Json(blog))
    }

    pub async fn delete(_path: Path<i32>, _app_state: State<AppState>) -> Result<Json<DeleteBlogResult>, BlogError> {
        // TODO: Delete a blog by id and return the deleted id, or NotFound if missing
        let id = Path.0;
        let result = sqlx::query_as!(
            r#"
                DELETE FROM blogs
                WHERE id = $1
                RETURNING id
            "#, 
            id
        )
        .fetch_one(&_app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BlogError::NotFound(id),
            _ => BlogError::Database(e),
        })?;

        Ok(Json(result))
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// get_by_id:
//   - fetch_optional + .ok_or(BlogError::NotFound(id))
//   - Look up: fetch_optional, map_err, RowNotFound
//
// get_all_by_author_id:
//   - "SELECT * FROM blogs WHERE author_id = $1 ORDER BY created_at DESC"
//   - fetch_all + Ok(Json(...))
//
// create:
//   - "INSERT INTO blogs (title, content, author_id) VALUES ($1, $2, $3) RETURNING *"
//   - fetch_one + return (StatusCode::CREATED, Json(blog))
//   - Look up: RETURNING *, StatusCode::CREATED
//
// update:
//   - "UPDATE blogs SET title = COALESCE($1, title), content = COALESCE($2, content), updated_at = NOW() WHERE id = $3 RETURNING *"
//   - fetch_optional + ok_or(BlogError::NotFound(id))
//   - Look up: COALESCE for partial updates, NOW()
//
// delete:
//   - "DELETE FROM blogs WHERE id = $1 RETURNING id"
//   - sqlx::query_as!(DeleteBlogResult, ...).fetch_optional(...).await?
//   - Look up: DELETE ... RETURNING, fetch_optional
