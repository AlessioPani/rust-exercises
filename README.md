# Rust Exercises
Repository containing learning materials, exercises and assessments for the Rust programming language.

## Cargo workspace

The exercises under `Exercises/` are members of a single Cargo workspace. Run these commands from the repository root:

```sh
# Build every exercise
cargo build --workspace

# Run one exercise by its package name
cargo run -p ex1_hello_world
```

## Exercise roadmap

### Existing exercises

1. `ex1_hello_world` — Hello World
2. `ex2_guessing_game` — guessing game
3. `ex3_temperature_converter` — temperature converter
4. `ex4_fibonacci_nth` — *n*th Fibonacci number
5. `ex5_christmas_carol` — lyrics of “The Twelve Days of Christmas”

### Planned exercises, in increasing difficulty

6. Menu-driven command-line calculator — `enum`, `match`, input parsing, and `Result`
7. Word counter for files — file reading, iterators, and `HashMap`
8. Hangman — state management, collections, and tests for game rules
9. JSON-backed to-do list — `struct`, `serde`, and file persistence
10. CSV expense analyzer — parsing, modules, and tests
11. Text search across files, like a small `grep` — CLI arguments, filesystem, and error handling
12. Client for a web API — HTTP requests, JSON, and asynchronous code
13. REST server for the to-do list — routing, HTTP, and persistence

To pass arguments to an exercise, add `--` after the package name, for example:

```sh
cargo run -p ex2_guessing_game -- --help
```
