use axum::Router;
// use std::net::TcpListener;
use tokio::net::TcpListener;

use crate::{app_state::AppState, domain::todo::routes::todo_routes};

mod app_state;
mod domain;

#[tokio::main]
async fn main() {
    // 1. App state.
    let app_state = AppState::default();
    let app = Router::new().merge(todo_routes()).with_state(app_state);

    // 2. Listen to specific port.
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    println!("Server is running on http://127.0.0.1:8000");

    // 3. Server.
    axum::serve(listener, app).await.unwrap();
}
