+++
title = "Tuple vs Struct"
description = "Comparing tuples and structs for data organization"
+++


## Background & Objectives

Understand the key differences between tuples and structs, and learn when to use each for organizing related data. This exercise demonstrates why named fields in structs provide better type safety and code clarity.

## Specs

This is a demonstration exercise with no TODOs. Run the program to see how:

1. Tuples allow accidental value swapping without compile-time errors
2. Structs prevent mistakes by requiring named fields
3. Named fields make code more readable and maintainable
4. The compiler catches errors when field names are used incorrectly

### Run the Program

```bash
cargo run
```

### Test Your Solution

```bash
cargo test
```

## Key Concepts

- Tuples use positional access (0, 1, 2) while structs use named fields
- Tuples can lead to bugs when values are accidentally swapped
- Structs provide better type safety through named fields
- Named fields improve code readability and self-documentation