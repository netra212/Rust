use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBlogDto {
    pub title: String,
    pub content: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBlogDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct DeleteBlogResult {
    pub id: i32,
}
