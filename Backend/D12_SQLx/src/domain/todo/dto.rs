// Import Serde traits:
// - Deserialize: allows converting JSON into Rust structs
// - Serialize: allows converting Rust structs into JSON
use serde::{Deserialize, Serialize};

/// DTO (Data Transfer Object) used when creating a new Todo.
/// This represents the JSON body sent by the client.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoDto {
    /// The text/content of the todo item.
    /// Example JSON:
    /// {
    ///   "text": "Buy milk"
    /// }
    pub text: String,
}

/// DTO used when updating an existing Todo.
///
/// Only fields that are `Some(...)` will be updated.
/// `None` means "do not change this field".
#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    /// Optional updated text for the todo.
    /// If `None`, the text remains unchanged.
    pub text: Option<String>,
}

/// Response returned after successfully deleting a todo.
///
/// This confirms which todo was deleted.
#[derive(Debug, Serialize)]
pub struct DeleteTodoResult {
    /// ID of the deleted todo item.
    /// Useful for client confirmation/logging.
    pub id: i32,
}
