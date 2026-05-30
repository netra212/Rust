+++
title = "Optional 02 User Database"
description = "Build a user database system with Result<Option<T>> patterns and custom error handling"
+++

## Background & Objectives

Build a complete user database system that demonstrates advanced error handling with `Result` and `Option` types. This exercise shows the powerful `Result<Option<T>>` pattern and how to convert between different error representations.

This exercise integrates concepts you've learned:
- Working with `Result<T, E>` for operations that can fail
- Using `Option<T>` for values that might not exist
- Combining both patterns: `Result<Option<T>>`
- Creating custom error types with enums
- Converting between `Option` and `Result`
- Validating data before operations
- Handling errors in batch operations

## Specs

### Project Structure

The project has four main files:
- `src/main.rs` - Main program demonstrating the database system
- `src/user.rs` - User struct and Status enum
- `src/error.rs` - Custom error types (DbError, UserError)
- `src/database.rs` - Database operations with error handling

### Your Tasks

#### In `src/user.rs`:

1. **TODO 1**: Define `Status` enum with variants: Active, Inactive, Suspended
2. **TODO 2**: Define `User` struct with fields:
   - id: u32
   - name: String
   - email: String
   - status: Status

3. **TODO 3**: Implement `User::new()` constructor
4. **TODO 4**: Implement `User::get_name()` - return reference to name
5. **TODO 5**: Implement `User::get_status()` - return reference to status
6. **TODO 6**: Implement `User::is_active()` - return true if status is Active
7. **TODO 7**: Implement `User::suspend()` - change status to Suspended

#### In `src/error.rs`:

1. **TODO 1**: Define `DbError` enum with variants:
   - ConnectionFailed
   - QueryFailed(String)
   - Timeout

2. **TODO 2**: Define `UserError` enum with variants:
   - NotFound
   - InvalidEmail
   - AlreadyExists
   - DatabaseError(DbError)

3. **TODO 3**: Implement `DbError::message()` - return descriptive error messages
4. **TODO 4**: Implement `UserError::message()` - return descriptive error messages, wrapping DbError when needed

#### In `src/database.rs`:

1. **TODO 1**: Implement `find_user()` - demonstrates `Result<Option<T>>` pattern
   - Find user by id
   - Returns `Ok(Some(user))` if found
   - Returns `Ok(None)` if not found
   - Returns `Err(DbError)` on database error

2. **TODO 2**: Implement `get_user()` - convert `Option` to `Result`
   - Use `find_user()` internally
   - Transform `Ok(None)` into `Err(UserError::NotFound)`

3. **TODO 3**: Implement `add_user()` - mutation with validation
   - Validate email format (must contain '@')
   - Check for duplicate ids
   - Add user to database

4. **TODO 4**: Implement `delete_user()` - remove and return user
   - Find user by id
   - Remove from Vec
   - Return the deleted user or error

5. **TODO 5**: Implement `get_users_or_error()` - batch operations
   - Retrieve multiple users by ids
   - All must exist or entire operation fails (fail-fast)

### How It Should Work

The program demonstrates:
1. Finding users with `Result<Option<T>>` pattern
2. Converting Option to Result for stricter error handling
3. Batch operations that fail if any user is missing
4. User deletion with proper error handling
5. Custom error types with nested errors (UserError wrapping DbError)

## Run & Test

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- **Result<Option<T>>**: The "double option" pattern for operations that can fail OR return nothing
- **Custom Error Types**: Creating domain-specific errors with enums
- **Error Conversion**: Transforming between `Option` and `Result`
- **Nested Errors**: Wrapping one error type inside another (UserError::DatabaseError(DbError))
- **Validation**: Checking data before operations (email format, duplicate ids)
- **Fail-Fast**: Stopping batch operations on first error
- **Pattern Matching**: Handling different error variants

## Learning Goals

This exercise teaches you to:
1. Understand when to use `Result<Option<T>>` vs `Result<T, E>`
2. Create meaningful custom error types
3. Convert between Option and Result appropriately
4. Handle errors at different levels (database vs application)
5. Implement validation with proper error reporting
6. Work with nested error types
7. Design error handling strategies (fail-fast vs continue on error)
