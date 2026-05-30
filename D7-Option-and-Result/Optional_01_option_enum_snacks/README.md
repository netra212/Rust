+++
title = "Option Enum Snacks"
description = "Working with Option types in practical scenarios"
+++


## Background & Objectives

This exercise provides a practical example of using Option types in a real-world scenario - a snack vending machine that might or might not have your desired snack.

## Specs

Study the complete implementation in `src/main.rs` to understand:

1. How a custom `MyOption` enum is defined with Some and None variants
2. How the `get_snack` function searches through an array and returns MyOption
3. How pattern matching extracts values from MyOption in the main function

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Practical use cases for Option types
- Modeling optional data with Option enum