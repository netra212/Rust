use axum::Json;
use axum::extract::{Path, Query, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::app_state::AppState;
use crate::domain::stats::error::StatsError;


/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuthorStats {
    pub author_id: i32,
    pub blog_count: i64,
    pub avg_length: Option<f64>,
    pub latest_blog: Option<String>,
}

impl AuthorStats {
    pub async fn get(
        Path(author_id): Path<i32>,
        State(app_state): State<AppState>,
    ) -> Result<Json<AuthorStats>, StatsError> {
        let stats = sqlx::query_as!(
            AuthorStats,
            r#"SELECT
                author_id  as "author_id!",
                blog_count as "blog_count!",
                avg_length,
                latest_blog
               FROM get_author_stats($1)"#,
            author_id
        )
        .fetch_optional(&app_state.pool)
        .await?
        .ok_or(StatsError::NotFound(author_id))?;

        Ok(Json(stats))
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BlogSearchResult {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

impl BlogSearchResult {
    pub async fn search(
        Query(params): Query<SearchQuery>,
        State(app_state): State<AppState>,
    ) -> Result<Json<Vec<BlogSearchResult>>, StatsError> {
        let blogs = sqlx::query_as!(
            BlogSearchResult,
            r#"SELECT * FROM blogs
               WHERE id IN (SELECT id FROM search_blogs($1))
               ORDER BY created_at DESC"#,
            params.q
        )
        .fetch_all(&app_state.pool)
        .await?;

        Ok(Json(blogs))
    }
}
