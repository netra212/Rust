+++
title = "Option Get Method Array"
description = "Using the get method with arrays to return Option types"
+++


## Background & Objectives

This exercise demonstrates how arrays' `get()` method returns an Option type, providing safe access to array elements without panicking.

## Specs

Complete the TODOs in `src/main.rs` to practice safe array access with Option:

1. `safe_get_sport`: Safely get a sport at the given index and return it as an owned String
2. `get_multiple_sports`: Get multiple sports by their indices, skip invalid indices
3. `find_sport_position`: Find the index of a specific sport in the array
4. `get_score_range`: Safely get a range of scores from start to end (exclusive)

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Using `get()` method for safe array access
- Option as a return type for potentially missing values