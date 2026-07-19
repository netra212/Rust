use axum::Router;
use axum::routing::get;

use crate::app_state::AppState;
use crate::domain::author::author::Author;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub fn authors_routes() -> Router<AppState> {
    Router::new()
        .route("/authors", get(Author::get_all).post(Author::create))
        .route(
            "/authors/{id}",
            get(Author::get_by_id).patch(Author::update).delete(Author::delete),
        )
}
