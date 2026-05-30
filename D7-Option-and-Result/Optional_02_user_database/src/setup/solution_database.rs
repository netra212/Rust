/* ========================================================== */
/*           🦀 DATABASE OPERATIONS 🦀                        */
/* ========================================================== */

use super::solution_error::{DbError, UserError};
use super::solution_user::User;

pub struct Database {
    pub users: Vec<User>,
}

impl Database {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    // === RESULT<OPTION<T>> PATTERN ===

    // TODO 1: Implement find_user() method
    // Find a user in the database by their id
    // This demonstrates the Result<Option<T>> pattern
    // The method can succeed with Some(user), succeed with None, or fail with an error
    // Look up Result<Option<T>> pattern in Rust documentation
    pub fn find_user(&self, id: u32) -> Result<Option<User>, DbError> {
        // Simulate connection error for id 999
        if id == 999 {
            return Err(DbError::ConnectionFailed);
        }

        Ok(self.users.iter().find(|u| u.id == id).cloned())
    }

    // === CONVERTING OPTION TO RESULT ===

    // TODO 2: Implement get_user() method
    // Get a user by id, treating "not found" as an error instead of None
    // Convert between Result<Option<T>> and Result<T, E>
    // Use the find_user method and transform its return type
    // Look up Option to Result conversion in Rust documentation
    pub fn get_user(&self, id: u32) -> Result<User, UserError> {
        self.find_user(id)
            .map_err(UserError::DatabaseError)?
            .ok_or(UserError::NotFound)
    }

    // === MUTATIONS WITH ERROR HANDLING ===

    // TODO 3: Implement add_user() method
    // Add a user to the database after validating the data
    // Prevent duplicate ids and validate email format
    // Look up Vec iteration and validation patterns in Rust documentation
    pub fn add_user(&mut self, user: User) -> Result<(), UserError> {
        // Check if user already exists
        if self.users.iter().any(|u| u.id == user.id) {
            return Err(UserError::AlreadyExists);
        }

        // Validate email
        if !user.email.contains('@') {
            return Err(UserError::InvalidEmail);
        }

        self.users.push(user);
        Ok(())
    }

    // TODO 4: Implement delete_user() method
    // Remove and return a user from the database by their id
    // Look up finding and removing elements from Vec in Rust documentation
    pub fn delete_user(&mut self, id: u32) -> Result<User, UserError> {
        let position = self
            .users
            .iter()
            .position(|u| u.id == id)
            .ok_or(UserError::NotFound)?;
        Ok(self.users.remove(position))
    }

    // === BATCH OPERATIONS WITH RESULT ===

    // TODO 5: Implement get_users_or_error() method
    // Retrieve multiple users by their ids
    // All users must exist or the entire operation fails (fail-fast)
    // Look up combining multiple Results in Rust documentation
    pub fn get_users_or_error(&self, ids: &[u32]) -> Result<Vec<User>, UserError> {
        ids.iter().map(|id| self.get_user(*id)).collect()
    }
}
