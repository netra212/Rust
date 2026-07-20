use axum::Router;
use axum::routing::get;

use crate::app_state::AppState;
use crate::domain::stats::stats::{AuthorStats, BlogSearchResult};

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */


pub fn stats_routes() -> Router<AppState> {
    Router::new()
        .route("/stats/author/{id}", get(AuthorStats::get))
        .route("/stats/search", get(BlogSearchResult::search))
}
