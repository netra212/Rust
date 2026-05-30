/* ========================================================== */
/*              🦀 CUSTOM ERROR TYPES 🦀                      */
/* ========================================================== */

// TODO 1: Define DbError enum with the following variants:
// - ConnectionFailed
// - QueryFailed(String) - holds an error message
// - Timeout
// Look up enum variants with data in Rust documentation
#[derive(Debug, PartialEq)]
pub enum DbError {
    ConnectionFailed,
    QueryFailed(String),
    Timeout,
}

// TODO 2: Define UserError enum with the following variants:
// - NotFound
// - InvalidEmail
// - AlreadyExists
// - DatabaseError(DbError) - wraps a DbError
// Look up enum variants that contain other enums in Rust documentation
#[derive(Debug, PartialEq)]
pub enum UserError {
    NotFound,
    InvalidEmail,
    AlreadyExists,
    DatabaseError(DbError),
}

impl DbError {
    // TODO 3: Implement message() method
    // Return a descriptive error message for each error type
    // Look up pattern matching on enum variants in Rust documentation
    pub fn message(&self) -> &str {
        match self {
            DbError::ConnectionFailed => "Database connection failed",
            DbError::QueryFailed(_) => "Query execution failed",
            DbError::Timeout => "Database operation timed out",
        }
    }
}

impl UserError {
    // TODO 4: Implement message() method
    // Return a descriptive error message for each error type
    // For DatabaseError, combine the error type with the inner error's message
    // Look up pattern matching with nested data in Rust documentation
    pub fn message(&self) -> String {
        match self {
            UserError::NotFound => "User not found".to_string(),
            UserError::InvalidEmail => "Invalid email format".to_string(),
            UserError::AlreadyExists => "User already exists".to_string(),
            UserError::DatabaseError(db_err) => {
                format!("Database error: {}", db_err.message())
            }
        }
    }
}
