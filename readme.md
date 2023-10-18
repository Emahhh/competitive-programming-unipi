# Competitive Programming

Repository containing my solutions to various exercises for the course "Competitive Programming and Contests" at the University of Pisa.

- Naming convention: `<lesson-number>_name.rs`
- After creating a new source file, rember to add it to the `Cargo.toml` file. This will enable VSCode's linting using `clippy`.


## Running 

To run your code in the terminal:
- Run `cargo build` to compile your code for the entire crate.
- Run `cargo run --bin bin_name` to run your program. Where `bin_name` is the name of your executable specified in the `Cargo.toml` file.
- Run `cargo test --bin bin_name` to run your tests.
  
Or use VSCode's "Run" button over any "main" function or "test" function.

## Useful commands

- use `cargo fmt` to format your code.
- use `cargo clippy` to check your code.

## Testing

### Running tests

- Run `cargo test --bin trees_handson ` to run all tests for the `trees_handson` binary.
- Run `cargo test --bin trees_handson -- --nocapture` to run tests without capturing output (so you can see the output in the terminal).

### Writing tests
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


