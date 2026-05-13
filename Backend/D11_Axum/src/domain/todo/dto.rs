// dto -> data transfer object.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}
