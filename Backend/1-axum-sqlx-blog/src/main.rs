#![forbid(clippy::unwrap_used)]

use axum::Router;

mod app_state;
mod domain;
mod utils;

use crate::app_state::AppState;
use crate::domain::author::routes::authors_routes;
use crate::domain::blog::routes::blogs_routes;
use crate::utils::setup_tracing::setup_tracing;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

const PORT_8000: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    setup_tracing();

    let app_state = AppState::init().await.expect("Failed to initialize database");

    let app = Router::new()
        .merge(authors_routes())
        .merge(blogs_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(PORT_8000)
        .await
        .expect("Failed to bind to address");

    tracing::debug!(
        "listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );

    axum::serve(listener, app).await.expect("Failed to start server");
}
