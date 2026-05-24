use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct TodoAPiError {
    pub text: String,
}
