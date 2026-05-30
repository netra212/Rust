#[cfg(test)]
mod setup;

mod user;
mod error;
mod database;

use user::User;
use database::Database;

/* ========================================================== */
/*                      🦀 MAIN 🦀                            */
/* ========================================================== */

fn main() {
    let mut db = Database::new();

    println!("=== User Database System ===\n");

    // Add some users
    let user1 = User::new(1, "Alice", "alice@example.com");
    let user2 = User::new(2, "Bob", "bob@example.com");
    let user3 = User::new(3, "Charlie", "charlie@example.com");

    println!("Adding users...");
    let _ = db.add_user(user1);
    let _ = db.add_user(user2);
    let _ = db.add_user(user3);

    // Demonstrate Result<Option<T>> pattern
    println!("\n=== Finding Users (Result<Option<T>>) ===");
    match db.find_user(1) {
        Ok(Some(user)) => println!("Found user: {} ({})", user.get_name(), user.email),
        Ok(None) => println!("User not found (but no error occurred)"),
        Err(err) => println!("Database error: {:?}", err),
    }

    match db.find_user(999) {
        Ok(Some(user)) => println!("Found user: {} ({})", user.get_name(), user.email),
        Ok(None) => println!("User not found (but no error occurred)"),
        Err(err) => println!("Database error: {:?}", err),
    }

    // Demonstrate converting Option to Result
    println!("\n=== Getting Users (Converting Option to Result) ===");
    match db.get_user(2) {
        Ok(user) => println!("Got user: {} - {}", user.get_name(), user.email),
        Err(err) => println!("Error: {}", err.message()),
    }

    match db.get_user(999) {
        Ok(user) => println!("Got user: {} - {}", user.get_name(), user.email),
        Err(err) => println!("Error: {}", err.message()),
    }

    // Batch operations
    println!("\n=== Batch Operations ===");
    match db.get_users_or_error(&[2, 3]) {
        Ok(users) => println!("Found {} users", users.len()),
        Err(err) => println!("Error: {}", err.message()),
    }

    match db.get_users_or_error(&[2, 999]) {
        Ok(users) => println!("Found {} users", users.len()),
        Err(err) => println!("Error: {}", err.message()),
    }

    // Delete a user
    println!("\n=== Deleting User ===");
    match db.delete_user(3) {
        Ok(user) => println!("Deleted user: {}", user.get_name()),
        Err(err) => println!("Error: {}", err.message()),
    }

    println!("\nRemaining users: {}", db.users.len());
}
