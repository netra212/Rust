/* ========================================================== */
/*           🦀 DATABASE OPERATIONS 🦀                        */
/* ========================================================== */

use crate::error::{DbError, UserError};
use crate::user::User;

pub struct Database {
    pub users: Vec<User>,
}

impl Database {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    // === RESULT<OPTION<T>> PATTERN ===

    // TODO 1: Implement find_user() — find by id; Ok(Some(user)), Ok(None), or Err(DbError)
    pub fn find_user(&self, _id: u32) -> Result<Option<User>, DbError> {
        Ok(None)
    }

    // === CONVERTING OPTION TO RESULT ===

    // TODO 2: Implement get_user() — use find_user() and treat "not found" as UserError::NotFound
    pub fn get_user(&self, _id: u32) -> Result<User, UserError> {
        Err(UserError::NotFound)
    }

    // === MUTATIONS WITH ERROR HANDLING ===

    // TODO 3: Implement add_user() — validate email and prevent duplicate ids before inserting
    pub fn add_user(&mut self, _user: User) -> Result<(), UserError> {
        Ok(())
    }

    // TODO 4: Implement delete_user() — remove and return the user by id, or error if not found
    pub fn delete_user(&mut self, _id: u32) -> Result<User, UserError> {
        Err(UserError::NotFound)
    }

    // === BATCH OPERATIONS WITH RESULT ===

    // TODO 5: Implement get_users_or_error() — all ids must exist or the whole operation fails
    pub fn get_users_or_error(&self, _ids: &[u32]) -> Result<Vec<User>, UserError> {
        Err(UserError::NotFound)
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// find_user:
//   - Use self.users.iter().find(|u| u.id == id) and wrap with Ok(...)
//   - Remember to clone the found user (or return a reference if you prefer)
//   - Look up: iter().find(), Option::cloned()
//
// get_user:
//   - Call find_user, then convert Ok(None) -> Err(UserError::NotFound)
//   - Look up: Result::and_then(), Option::ok_or()
//
// add_user:
//   - Check if any existing user has the same id -> AlreadyExists
//   - Validate email contains '@' -> InvalidEmail
//   - Then push the user
//   - Look up: iter().any(), push(), early return
//
// delete_user:
//   - Find the position with iter().position(), then remove it with Vec::remove()
//   - Return the removed user
//   - Look up: Vec::remove(), position()
//
// get_users_or_error:
//   - For each id, call get_user(); if any fails, return that error immediately
//   - Use ? or collect into Result<Vec<_>, _>
//   - Look up: Iterator::collect() with Result, ? operator
