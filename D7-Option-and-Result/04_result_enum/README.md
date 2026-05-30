+++
title = "Result Divide"
description = "Using Result for division operations that might fail"
+++


## Background & Objectives

This exercise demonstrates how to use Result for operations that can fail, teaching you to create your own Result-like enum to understand error handling.

## Specs

Complete the TODOs in `src/main.rs` to work with a custom Result enum:

1. `create_success`: Create a successful result containing the given value
2. `validate_age`: Validate age is between 0 and 120, return appropriate error message
3. `parse_number`: Parse string to number, return error if invalid
4. `divide_numbers`: Divide a by b, return error if b is zero

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Returning Result from functions that can fail
- Handling division by zero with Result types