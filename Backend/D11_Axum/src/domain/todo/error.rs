use serde::Serialize;

/**
* Used for JSON Error:
*   {
        "error": "Todo with id 5 not found"
    }
 *
 *
*/
#[derive(Debug, Serialize)]
pub struct TodoApiError {
    pub error: String,
}
