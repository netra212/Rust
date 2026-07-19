use axum::Router;
use axum::routing::get;

use crate::app_state::AppState;
use crate::domain::blog::blog::Blog;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn blogs_routes() -> Router<AppState> {
    Router::new()
        .route("/blogs", get(Blog::get_all).post(Blog::create))
        .route(
            "/blogs/{id}",
            get(Blog::get_by_id).patch(Blog::update).delete(Blog::delete),
        )
        .route("/authors/{id}/blogs", get(Blog::get_all_by_author_id))
}
