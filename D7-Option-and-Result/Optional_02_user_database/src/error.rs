/* ========================================================== */
/*              🦀 CUSTOM ERROR TYPES 🦀                      */
/* ========================================================== */

// TODO 1: Define DbError enum with variants: ConnectionFailed, QueryFailed(String), Timeout
#[derive(Debug, PartialEq)]
pub enum DbError {
    ConnectionFailed,
    QueryFailed(String),
    Timeout,
}

// TODO 2: Define UserError enum with variants: NotFound, InvalidEmail, AlreadyExists, DatabaseError(DbError)
#[derive(Debug, PartialEq)]
pub enum UserError {
    NotFound,
    InvalidEmail,
    AlreadyExists,
    DatabaseError(DbError),
}

impl DbError {
    // TODO 3: Implement message() — return a descriptive &str for each variant
    pub fn message(&self) -> &str {
        match self {
            DbError::ConnectionFailed => "Database connection failed",
            DbError::QueryFailed(_) => "",  // TODO: Add appropriate message
            DbError::Timeout => "",  // TODO: Add appropriate message
        }
    }
}

impl UserError {
    // TODO 4: Implement message() — return a String; for DatabaseError include the inner message
    pub fn message(&self) -> String {
        match self {
            UserError::NotFound => "User not found".to_string(),
            UserError::InvalidEmail => String::new(),  // TODO: Add appropriate message
            UserError::AlreadyExists => String::new(),  // TODO: Add appropriate message
            UserError::DatabaseError(_db_err) => String::new(),  // TODO: Include wrapped error message
        }
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// DbError::message():
//   - Match each variant and return a static string slice
//   - For QueryFailed(msg) you can include the msg or ignore it
//   - Look up: match, &str return type
//
// UserError::message():
//   - For simple variants return a descriptive String
//   - For DatabaseError(db_err), call db_err.message() and include it in the output
//   - Look up: format!(), nested pattern matching, to_string()
