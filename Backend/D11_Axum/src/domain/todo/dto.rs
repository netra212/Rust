// dto -> Data Transfer Object.
/**
 * Used for:
 * request payloads
 * response payloads
 * POST /todos

    {
    "text": "Learn Rust"
    }

 * Here, Data will comes in the form of JSON like as shown in above, but because of this Deserialize, this gets converted into struct.
 * Who does this converstion ?
 * Serde does the conversion.
 *
 * Why separate DTO ?
 * Because request data is NOT always same as database model.
 * Example:
 * Database Model:
  Todo {
        id,
        text
    }
 * Client sends:
 * JSON
    {
        "text": "abc"
    }
 *
 * No id. So DTO separates:
    -> API input
    -> internal model
*/
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}
