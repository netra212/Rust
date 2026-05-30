+++
title = "Option Your Own Enum"
description = "Creating your own Option-like enum to understand how Option works"
+++


## Background & Objectives

This exercise teaches you how to create your own Option-like enum to understand the underlying concepts of how Rust's built-in Option type works.

## Specs

Complete the TODOs in `src/main.rs` to work with a custom Option enum:

1. `create_some_value`: Create a MyOption::Some containing the given value
2. `double_if_some`: Double the value if Some, return None if None
3. `combine_options`: Add the values if both are Some, return None otherwise
4. `convert_to_string`: Convert the integer to string if Some, preserve None

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Creating generic enums with type parameters
- Pattern matching with custom enum variants