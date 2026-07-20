use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAuthorDto {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAuthorDto {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteAuthorResult {
    pub id: i32,
}
