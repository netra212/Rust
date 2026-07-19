use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoDto {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteTodoResult {
    pub id: i32,
}
