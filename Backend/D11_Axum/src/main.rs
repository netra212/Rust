use axum::Router;
use tokio::net::TcpListener;

use crate::{app_state::AppState, domain::todo::routes::todo_routes};

mod app_state;
mod domain;

#[tokio::main]
async fn main() {
    //
    // 1. App state.
    let app_state = AppState::default(); // Arc<RwLock<HashMap<i32, Todo>>> --> Meaning --> Shared in-memory database
    // Arc<RwLock<HashMap<i32, Todo>>> <-- It means we are allowing to have many readers but allowing to write only one times.

    /* Router::new() -> Create empty router.
    merge(todo_routes()) -> Import routes from todo module Or Equivalent mentally -> Attach todo API routes here
    with_state(app_state) -> Attach shared application state which means every route can access this route. This transforms router type into: Router<AppState>
    */
    let app = Router::new().merge(todo_routes()).with_state(app_state);

    // 2. Listen to specific port.
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    println!("Server is running on http://127.0.0.1:8000");

    // 3. Server.
    /*
    this code axum::serve(listener, app).await.unwrap(); means
        - starts HTTP server
        - waits forever
        - handles requests asynchronously
    */
    axum::serve(listener, app).await.unwrap();
}
