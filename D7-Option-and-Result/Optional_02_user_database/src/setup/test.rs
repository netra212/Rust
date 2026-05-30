#[cfg(feature = "solution")]
#[path = "solution_user.rs"]
mod solution_user;

#[cfg(feature = "solution")]
#[path = "solution_error.rs"]
mod solution_error;

#[cfg(feature = "solution")]
#[path = "solution_database.rs"]
mod solution_database;

#[cfg(feature = "solution")]
use solution_user::{Status, User};
#[cfg(feature = "solution")]
use solution_error::{DbError, UserError};
#[cfg(feature = "solution")]
use solution_database::Database;

#[cfg(not(feature = "solution"))]
use crate::user::{Status, User};
#[cfg(not(feature = "solution"))]
use crate::error::{DbError, UserError};
#[cfg(not(feature = "solution"))]
use crate::database::Database;

// === User Tests ===

#[test]
fn test_user_creation() {
    let user = User::new(1, "Alice", "alice@example.com");
    assert_eq!(user.id, 1);
    assert_eq!(user.get_name(), "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.get_status(), &Status::Active);
}

#[test]
fn test_user_is_active() {
    let user = User::new(1, "Alice", "alice@example.com");
    assert!(user.is_active());
}

#[test]
fn test_user_suspend() {
    let mut user = User::new(1, "Alice", "alice@example.com");
    user.suspend();
    assert_eq!(user.get_status(), &Status::Suspended);
    assert!(!user.is_active());
}

// === Error Tests ===

#[test]
fn test_user_error_messages() {
    assert_eq!(UserError::NotFound.message(), "User not found");
    assert_eq!(UserError::InvalidEmail.message(), "Invalid email format");
    assert_eq!(UserError::AlreadyExists.message(), "User already exists");
    assert!(UserError::DatabaseError(DbError::ConnectionFailed).message().contains("Database error"));
}

// === Database Tests - Result<Option<T>> Pattern ===

#[test]
fn test_find_user_found() {
    let mut db = Database::new();
    let user = User::new(1, "Alice", "alice@example.com");
    let _ = db.add_user(user.clone());

    let result = db.find_user(1);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_find_user_not_found() {
    let db = Database::new();
    let result = db.find_user(1);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_find_user_connection_error() {
    let db = Database::new();
    let result = db.find_user(999);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DbError::ConnectionFailed);
}

// === Converting Option to Result ===

#[test]
fn test_get_user_found() {
    let mut db = Database::new();
    let user = User::new(1, "Alice", "alice@example.com");
    let _ = db.add_user(user);

    let result = db.get_user(1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get_name(), "Alice");
}

#[test]
fn test_get_user_not_found() {
    let db = Database::new();
    let result = db.get_user(1);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UserError::NotFound);
}

#[test]
fn test_get_user_database_error() {
    let db = Database::new();
    let result = db.get_user(999);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UserError::DatabaseError(DbError::ConnectionFailed));
}

// === Mutations with Error Handling ===

#[test]
fn test_add_user_success() {
    let mut db = Database::new();
    let user = User::new(1, "Alice", "alice@example.com");
    let result = db.add_user(user);
    assert!(result.is_ok());
    assert_eq!(db.users.len(), 1);
}

#[test]
fn test_add_user_already_exists() {
    let mut db = Database::new();
    let user1 = User::new(1, "Alice", "alice@example.com");
    let user2 = User::new(1, "Bob", "bob@example.com");

    let _ = db.add_user(user1);
    let result = db.add_user(user2);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UserError::AlreadyExists);
}

#[test]
fn test_delete_user_success() {
    let mut db = Database::new();
    let user = User::new(1, "Alice", "alice@example.com");
    let _ = db.add_user(user);

    let result = db.delete_user(1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get_name(), "Alice");
    assert_eq!(db.users.len(), 0);
}

// === Batch Operations ===

#[test]
fn test_get_users_or_error_all_found() {
    let mut db = Database::new();
    let _ = db.add_user(User::new(1, "Alice", "alice@example.com"));
    let _ = db.add_user(User::new(2, "Bob", "bob@example.com"));

    let result = db.get_users_or_error(&[1, 2]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_get_users_or_error_one_not_found() {
    let mut db = Database::new();
    let _ = db.add_user(User::new(1, "Alice", "alice@example.com"));

    let result = db.get_users_or_error(&[1, 2]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UserError::NotFound);
}
