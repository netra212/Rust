use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;
use W3_D13_1_sqlx_blog::app_state::AppState;
use W3_D13_1_sqlx_blog::domain::author::author::Author;
use W3_D13_1_sqlx_blog::domain::author::routes::authors_routes;

// #[sqlx::test] injects a fresh, isolated Postgres database for each test.
// Migrations run automatically. The database is dropped when the test ends.
// Your local dev database is never touched.
fn create_test_app(pool: PgPool) -> Router {
    let app_state = AppState { pool };

    Router::new()
        .merge(authors_routes())
        .with_state(app_state)
}

async fn send_request(app: Router, request: Request<Body>) -> (StatusCode, String) {
    let response = app.oneshot(request).await.expect("Failed to send request");
    let status = response.status();
    let body = response.into_body().collect().await.expect("Failed to collect body").to_bytes();
    let body_str = String::from_utf8(body.to_vec()).expect("Failed to parse body as UTF-8");
    (status, body_str)
}

/* ========================================================== */
/*                    ✨ GET /authors ✨                     */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_get_all_authors(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .uri("/authors")
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::OK);
    let authors: Vec<Author> = serde_json::from_str(&body).expect("Failed to parse authors");
    // Should have the seed authors from migration
    assert!(!authors.is_empty());
}

/* ========================================================== */
/*                  ✨ GET /authors/:id ✨                   */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_get_author_by_id_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let create_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Test Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, create_body) = send_request(app.clone(), create_request).await;
    let created_author: Author = serde_json::from_str(&create_body).expect("Failed to parse author");

    // Now get that specific author by ID
    let request = Request::builder()
        .uri(&format!("/authors/{}", created_author.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::OK);
    let author: Author = serde_json::from_str(&body).expect("Failed to parse author");
    assert_eq!(author.id, created_author.id);
    assert_eq!(author.name, "Test Author");
}

#[sqlx::test(migrations = "./migrations")]
async fn test_get_author_by_id_not_found(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .uri("/authors/999999")
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    let error: Value = serde_json::from_str(&body).expect("Failed to parse error");
    assert!(error["error"].as_str().expect("Error field missing").contains("not found"));
}

/* ========================================================== */
/*                    ✨ POST /authors ✨                    */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_create_author_success(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Jane Doe"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    let author: Author = serde_json::from_str(&body).expect("Failed to parse author");
    assert_eq!(author.name, "Jane Doe");
    assert!(author.id > 0);
}

/* ========================================================== */
/*                 ✨ PATCH /authors/:id ✨                  */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_update_author_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let create_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Original Name"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, body) = send_request(app.clone(), create_request).await;
    let created_author: Author = serde_json::from_str(&body).expect("Failed to parse author");

    // Now update it
    let update_request = Request::builder()
        .method("PATCH")
        .uri(&format!("/authors/{}", created_author.id))
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Updated Name"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (status, body) = send_request(app, update_request).await;

    assert_eq!(status, StatusCode::OK);
    let updated_author: Author = serde_json::from_str(&body).expect("Failed to parse author");
    assert_eq!(updated_author.id, created_author.id);
    assert_eq!(updated_author.name, "Updated Name");
    assert!(updated_author.updated_at > created_author.updated_at);
}

#[sqlx::test(migrations = "./migrations")]
async fn test_update_author_not_found(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .method("PATCH")
        .uri("/authors/999999")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Updated Name"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    let error: Value = serde_json::from_str(&body).expect("Failed to parse error");
    assert!(error["error"].as_str().expect("Error field missing").contains("not found"));
}

/* ========================================================== */
/*                ✨ DELETE /authors/:id ✨                  */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_delete_author_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let create_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "To Be Deleted"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, body) = send_request(app.clone(), create_request).await;
    let created_author: Author = serde_json::from_str(&body).expect("Failed to parse author");

    // Now delete it
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(&format!("/authors/{}", created_author.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app.clone(), delete_request).await;

    assert_eq!(status, StatusCode::OK);
    let result: Value = serde_json::from_str(&body).expect("Failed to parse result");
    assert_eq!(result["id"].as_i64().expect("ID field missing"), created_author.id as i64);

    // Verify it's actually deleted
    let get_request = Request::builder()
        .uri(&format!("/authors/{}", created_author.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, _) = send_request(app, get_request).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "./migrations")]
async fn test_delete_author_not_found(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .method("DELETE")
        .uri("/authors/999999")
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    let error: Value = serde_json::from_str(&body).expect("Failed to parse error");
    assert!(error["error"].as_str().expect("Error field missing").contains("not found"));
}

/* ========================================================== */
/*            ✨ INTEGRATION FLOW TESTS ✨                   */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_full_crud_flow_authors(pool: PgPool) {
    let app = create_test_app(pool);

    // 1. CREATE a new author
    let create_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "CRUD Test Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (create_status, create_body) = send_request(app.clone(), create_request).await;
    assert_eq!(create_status, StatusCode::CREATED);
    let created_author: Author = serde_json::from_str(&create_body).expect("Failed to parse author");
    assert_eq!(created_author.name, "CRUD Test Author");

    // 2. READ the created author
    let read_request = Request::builder()
        .uri(&format!("/authors/{}", created_author.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (read_status, read_body) = send_request(app.clone(), read_request).await;
    assert_eq!(read_status, StatusCode::OK);
    let read_author: Author = serde_json::from_str(&read_body).expect("Failed to parse author");
    assert_eq!(read_author.id, created_author.id);
    assert_eq!(read_author.name, "CRUD Test Author");

    // 3. UPDATE the author
    let update_request = Request::builder()
        .method("PATCH")
        .uri(&format!("/authors/{}", created_author.id))
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "CRUD Updated Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (update_status, update_body) = send_request(app.clone(), update_request).await;
    assert_eq!(update_status, StatusCode::OK);
    let updated_author: Author = serde_json::from_str(&update_body).expect("Failed to parse author");
    assert_eq!(updated_author.name, "CRUD Updated Author");

    // 4. DELETE the author
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(&format!("/authors/{}", created_author.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (delete_status, _) = send_request(app.clone(), delete_request).await;
    assert_eq!(delete_status, StatusCode::OK);

    // 5. VERIFY it's deleted
    let verify_request = Request::builder()
        .uri(&format!("/authors/{}", created_author.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (verify_status, _) = send_request(app, verify_request).await;
    assert_eq!(verify_status, StatusCode::NOT_FOUND);
}
