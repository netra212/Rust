use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoApiError {
    pub error: String,
}
