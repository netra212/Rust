use axum::Router;
use axum::routing::get;

use crate::app_state::AppState;
use crate::domain::todo::todo::Todo;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */


pub fn todos_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(Todo::get_all).post(Todo::create))
        .route(
            "/todos/{id}",
            get(Todo::get_by_id).patch(Todo::update).delete(Todo::delete),
        )
}
