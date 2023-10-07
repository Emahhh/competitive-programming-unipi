# Competitive Programming

Repository containing my solutions to various exercises for the course "Competitive Programming and Contests" at the University of Pisa.

- Naming convention: `<lesson-number>_name.rs`
- After creating a new source file, rember to add it to the `Cargo.toml` file. This will enable VSCode's linting using `clippy`.

## Useful commands

- use `cargo fmt` to format your code.
- use `cargo clippy` to check your code.

## Testing

Write some tests in the same file using this format:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_multiplication() {
        assert_eq!(2 * 3, 6);
    }

    #[test]
    fn test_division() {
        assert_eq!(10 / 2, 5);
    }
}
```

Some commonly used **assertion** macros include:
- `assert_eq!(left, right)` - Asserts that `left` is equal to `right`.
- `assert_ne!(left, right)` - Asserts that `left` is not equal to `right`.
- `assert!(condition)` - Asserts that `condition` is true.
- `assert!(condition, message)` - Asserts that `condition` is true and displays the `message` if the assertion fails.

Finally, run `cargo test` or use VSCode's "tests" button.

