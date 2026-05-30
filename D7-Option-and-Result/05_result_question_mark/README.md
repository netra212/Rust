+++
title = "Result Question Mark"
description = "Using the ? operator for error propagation"
+++


## Background & Objectives

This exercise teaches you about the question mark operator (?) for elegant error propagation in Rust, reducing boilerplate code.

## Specs

Complete the TODOs in `src/main.rs` to practice error propagation with the ? operator:

1. `parse_user_name`: Parse name from format "name:Alice", return error if invalid format
2. `parse_user_age`: Parse age from format "age:25", return error if invalid format or number
3. `parse_user_email`: Parse email from format "email:alice@example.com", validate contains @
4. `parse_complete_user`: Parse all three inputs and create a User, propagate any errors

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Using the `?` operator for error propagation
- Simplifying error handling code with question mark syntax