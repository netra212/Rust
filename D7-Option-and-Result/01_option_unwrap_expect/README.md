+++
title = "Option Unwrap and Expect"
description = "Using unwrap and expect methods with Option types"
+++


## Background & Objectives

This exercise teaches you about the Option type and the `unwrap()` and `expect()` methods for extracting values, including their panic behavior.

## Specs

Complete the TODOs in `src/main.rs` to practice working with Option methods:

1. `get_score_with_panic`: Extract the score at the given index, panic if None or out of bounds
2. `get_score_with_message`: Extract the score at the given index with a custom panic message
3. `get_score_or_default`: Extract the score or return the default value if None
4. `parse_number_or_panic`: Parse the string to number, panic with message if None or invalid

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Using `unwrap()` and `expect()` to extract Option values
- Understanding panic behavior with None values