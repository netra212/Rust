use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;
use W3_D13_1_sqlx_blog::app_state::AppState;
use W3_D13_1_sqlx_blog::domain::author::routes::authors_routes;
use W3_D13_1_sqlx_blog::domain::blog::blog::Blog;
use W3_D13_1_sqlx_blog::domain::blog::routes::blogs_routes;

// #[sqlx::test] injects a fresh, isolated Postgres database for each test.
// Migrations run automatically. The database is dropped when the test ends.
// Your local dev database is never touched.
fn create_test_app(pool: PgPool) -> Router {
    let app_state = AppState { pool };

    Router::new()
        .merge(authors_routes())
        .merge(blogs_routes())
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
/*                     ✨ GET /blogs ✨                      */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_get_all_blogs(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .uri("/blogs")
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::OK);
    let blogs: Vec<Blog> = serde_json::from_str(&body).expect("Failed to parse blogs");
    // Should have the seed blogs from migration
    assert!(!blogs.is_empty());
}

/* ========================================================== */
/*                   ✨ GET /blogs/:id ✨                    */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_get_blog_by_id_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let author_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Blog Test Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, author_body) = send_request(app.clone(), author_request).await;
    let author: Value = serde_json::from_str(&author_body).expect("Failed to parse author");
    let author_id = author["id"].as_i64().expect("Author ID missing") as i32;

    // Create a blog
    let create_request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "Test Blog",
                "content": "Test Content",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, create_body) = send_request(app.clone(), create_request).await;
    let created_blog: Blog = serde_json::from_str(&create_body).expect("Failed to parse blog");

    // Now get that specific blog by ID
    let request = Request::builder()
        .uri(&format!("/blogs/{}", created_blog.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::OK);
    let blog: Blog = serde_json::from_str(&body).expect("Failed to parse blog");
    assert_eq!(blog.id, created_blog.id);
    assert_eq!(blog.title, "Test Blog");
}

#[sqlx::test(migrations = "./migrations")]
async fn test_get_blog_by_id_not_found(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .uri("/blogs/999999")
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    let error: Value = serde_json::from_str(&body).expect("Failed to parse error");
    assert!(error["error"].as_str().expect("Error field missing").contains("not found"));
}

/* ========================================================== */
/*              ✨ GET /authors/:id/blogs ✨                 */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_get_blogs_by_author_id(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let author_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Author With Blogs"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, author_body) = send_request(app.clone(), author_request).await;
    let author: Value = serde_json::from_str(&author_body).expect("Failed to parse author");
    let author_id = author["id"].as_i64().expect("Author ID missing") as i32;

    // Create first blog
    let blog1_request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "Blog 1",
                "content": "Content 1",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");
    send_request(app.clone(), blog1_request).await;

    // Create second blog
    let blog2_request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "Blog 2",
                "content": "Content 2",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");
    send_request(app.clone(), blog2_request).await;

    // Get all blogs by this author
    let request = Request::builder()
        .uri(&format!("/authors/{}/blogs", author_id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::OK);
    let blogs: Vec<Blog> = serde_json::from_str(&body).expect("Failed to parse blogs");
    assert_eq!(blogs.len(), 2);
    assert!(blogs.iter().all(|b| b.author_id == author_id));
}

/* ========================================================== */
/*                    ✨ POST /blogs ✨                      */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_create_blog_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let author_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Blog Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, author_body) = send_request(app.clone(), author_request).await;
    let author: Value = serde_json::from_str(&author_body).expect("Failed to parse author");
    let author_id = author["id"].as_i64().expect("Author ID missing") as i32;

    // Create blog
    let request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "New Blog Post",
                "content": "This is the content of the blog post",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (status, body) = send_request(app, request).await;

    assert_eq!(status, StatusCode::CREATED);
    let blog: Blog = serde_json::from_str(&body).expect("Failed to parse blog");
    assert_eq!(blog.title, "New Blog Post");
    assert_eq!(blog.content, "This is the content of the blog post");
    assert_eq!(blog.author_id, author_id);
}

/* ========================================================== */
/*                  ✨ PATCH /blogs/:id ✨                   */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_update_blog_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let author_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Update Test Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, author_body) = send_request(app.clone(), author_request).await;
    let author: Value = serde_json::from_str(&author_body).expect("Failed to parse author");
    let author_id = author["id"].as_i64().expect("Author ID missing") as i32;

    // Create a blog
    let create_request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "Original Title",
                "content": "Original Content",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, body) = send_request(app.clone(), create_request).await;
    let created_blog: Blog = serde_json::from_str(&body).expect("Failed to parse blog");

    // Now update it
    let update_request = Request::builder()
        .method("PATCH")
        .uri(&format!("/blogs/{}", created_blog.id))
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "Updated Title",
                "content": "Updated Content"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (status, body) = send_request(app, update_request).await;

    assert_eq!(status, StatusCode::OK);
    let updated_blog: Blog = serde_json::from_str(&body).expect("Failed to parse blog");
    assert_eq!(updated_blog.id, created_blog.id);
    assert_eq!(updated_blog.title, "Updated Title");
    assert_eq!(updated_blog.content, "Updated Content");
    assert_eq!(updated_blog.author_id, author_id);
    assert!(updated_blog.updated_at > created_blog.updated_at);
}

#[sqlx::test(migrations = "./migrations")]
async fn test_update_blog_not_found(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .method("PATCH")
        .uri("/blogs/999999")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "Updated Title"
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
/*                 ✨ DELETE /blogs/:id ✨                   */
/* ========================================================== */

#[sqlx::test(migrations = "./migrations")]
async fn test_delete_blog_success(pool: PgPool) {
    let app = create_test_app(pool);

    // First create an author
    let author_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "Delete Test Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, author_body) = send_request(app.clone(), author_request).await;
    let author: Value = serde_json::from_str(&author_body).expect("Failed to parse author");
    let author_id = author["id"].as_i64().expect("Author ID missing") as i32;

    // Create a blog
    let create_request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "To Be Deleted",
                "content": "Delete Content",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, body) = send_request(app.clone(), create_request).await;
    let created_blog: Blog = serde_json::from_str(&body).expect("Failed to parse blog");

    // Now delete it
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(&format!("/blogs/{}", created_blog.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, body) = send_request(app.clone(), delete_request).await;

    assert_eq!(status, StatusCode::OK);
    let result: Value = serde_json::from_str(&body).expect("Failed to parse result");
    assert_eq!(result["id"].as_i64().expect("ID field missing"), created_blog.id as i64);

    // Verify it's actually deleted
    let get_request = Request::builder()
        .uri(&format!("/blogs/{}", created_blog.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (status, _) = send_request(app, get_request).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "./migrations")]
async fn test_delete_blog_not_found(pool: PgPool) {
    let app = create_test_app(pool);

    let request = Request::builder()
        .method("DELETE")
        .uri("/blogs/999999")
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
async fn test_full_crud_flow_blogs(pool: PgPool) {
    let app = create_test_app(pool);

    // 0. CREATE an author first
    let author_request = Request::builder()
        .method("POST")
        .uri("/authors")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "name": "CRUD Flow Author"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (_, author_body) = send_request(app.clone(), author_request).await;
    let author: Value = serde_json::from_str(&author_body).expect("Failed to parse author");
    let author_id = author["id"].as_i64().expect("Author ID missing") as i32;

    // 1. CREATE a new blog
    let create_request = Request::builder()
        .method("POST")
        .uri("/blogs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "CRUD Test Blog",
                "content": "CRUD Test Content",
                "author_id": author_id
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (create_status, create_body) = send_request(app.clone(), create_request).await;
    assert_eq!(create_status, StatusCode::CREATED);
    let created_blog: Blog = serde_json::from_str(&create_body).expect("Failed to parse blog");
    assert_eq!(created_blog.title, "CRUD Test Blog");

    // 2. READ the created blog
    let read_request = Request::builder()
        .uri(&format!("/blogs/{}", created_blog.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (read_status, read_body) = send_request(app.clone(), read_request).await;
    assert_eq!(read_status, StatusCode::OK);
    let read_blog: Blog = serde_json::from_str(&read_body).expect("Failed to parse blog");
    assert_eq!(read_blog.id, created_blog.id);
    assert_eq!(read_blog.title, "CRUD Test Blog");

    // 3. UPDATE the blog
    let update_request = Request::builder()
        .method("PATCH")
        .uri(&format!("/blogs/{}", created_blog.id))
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "title": "CRUD Updated Blog"
            })
            .to_string(),
        ))
        .expect("Failed to build request");

    let (update_status, update_body) = send_request(app.clone(), update_request).await;
    assert_eq!(update_status, StatusCode::OK);
    let updated_blog: Blog = serde_json::from_str(&update_body).expect("Failed to parse blog");
    assert_eq!(updated_blog.title, "CRUD Updated Blog");
    assert_eq!(updated_blog.content, "CRUD Test Content"); // Should remain unchanged

    // 4. DELETE the blog
    let delete_request = Request::builder()
        .method("DELETE")
        .uri(&format!("/blogs/{}", created_blog.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (delete_status, _) = send_request(app.clone(), delete_request).await;
    assert_eq!(delete_status, StatusCode::OK);

    // 5. VERIFY it's deleted
    let verify_request = Request::builder()
        .uri(&format!("/blogs/{}", created_blog.id))
        .body(Body::empty())
        .expect("Failed to build request");

    let (verify_status, _) = send_request(app, verify_request).await;
    assert_eq!(verify_status, StatusCode::NOT_FOUND);
}
