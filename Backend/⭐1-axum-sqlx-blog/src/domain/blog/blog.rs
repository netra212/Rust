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

    pub async fn get_by_id(Path(id): Path<i32>, app_state: State<AppState>) -> Result<Json<Blog>, BlogError> {
        let blog = sqlx::query_as!(
            Blog,
            r#"
                SELECT * 
                FROM blogs 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BlogError::NotFound(id),
            _ => BlogError::Database(e),
        })?;

        Ok(Json(blog))
    }

    pub async fn get_all_by_author_id(
        Path(author_id): Path<i32>,
        app_state: State<AppState>,
    ) -> Result<Json<Vec<Blog>>, BlogError> {
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
        app_state: State<AppState>,
        Json(body): Json<CreateBlogDto>,
    ) -> Result<(StatusCode, Json<Blog>), BlogError> {
        let blog = sqlx::query_as!(
            Blog,
            r#"
                INSERT INTO blogs (title, content, author_id) 
                VALUES ($1, $2, $3) 
                RETURNING *
            "#,
            body.title,
            body.content,
            body.author_id
        )
        .fetch_one(&app_state.pool)
        .await?;

        Ok((StatusCode::CREATED, Json(blog)))
    }

    pub async fn update(
        Path(id): Path<i32>,
        app_state: State<AppState>,
        Json(body): Json<UpdateBlogDto>,
    ) -> Result<Json<Blog>, BlogError> {
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
            body.title,
            body.content,
            body.author_id,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BlogError::NotFound(id),
            _ => BlogError::Database(e),
        })?;

        Ok(Json(blog))
    }

    pub async fn delete(Path(id): Path<i32>, app_state: State<AppState>) -> Result<Json<DeleteBlogResult>, BlogError> {
        let result = sqlx::query_as!(
            DeleteBlogResult,
            r#"
                DELETE FROM blogs 
                WHERE id = $1
                RETURNING id
            "#,
            id
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => BlogError::NotFound(id),
            _ => BlogError::Database(e),
        })?;

        Ok(Json(result))
    }
}
