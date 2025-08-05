# Contributing to N-dimensional Minesweeper

First off, thank you for considering contributing to this project! This is a fun, educational project, and we welcome contributions that align with that spirit.

## Project Philosophy

The primary goal of this project is to be a learning tool. We want the code to be as clear and understandable as possible, even for those who are new to Rust or game development. This means that maintainability, clarity, and extensibility are our top priorities.

## How to Contribute

1.  **Fork the repository.**
2.  **Create a new branch** for your feature or bug fix.
3.  **Write your code!**
4.  **Submit a pull request.**

## Coding Style and Guidelines

### 1. Verbose, Educational Comments are a MUST

This is the most important rule. Every piece of code you write should be accompanied by comments that explain *why* the code is written the way it is. Don't just explain *what* the code does (the code itself should do that); explain the reasoning behind your decisions.

**Good Comment Example:**

```rust
// We use a flat vector to store the board's cells, rather than a nested
// structure. This is more memory-efficient and often faster to access,
// especially in a highly dimensional space. The `coordinates` module
// provides functions to map N-dimensional coordinates to a 1D index.
let cells = Vec::with_capacity(total_cells);
```

**Bad Comment Example:**

```rust
// Create a vector for the cells.
let cells = Vec::with_capacity(total_cells);
```

### 2. Follow Standard Rust Conventions

Please adhere to the standard Rust formatting and style guidelines. Run `cargo fmt` before committing your changes.

### 3. Write Tests

For any new logic you add, please include tests. This helps ensure that your code works correctly and protects against future regressions. All tests should be augmented, and removing any test requires a thorough explanation (including replacement test(s)).

### 4. Keep it Extensible

When adding new features, think about how they might be extended in the future. For example, if you're adding a new type of cell, consider how other developers might add their own custom cell types later on. A modular, decoupled architecture is key.

Thank you for helping make this project a great resource for learning!
