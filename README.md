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

Available packages:

- `ex1_hello_world`
- `ex2_guessing_game`
- `ex3_temperature_converter`
- `ex4_fibonacci_nth`
- `ex5_christmas_carol`

To pass arguments to an exercise, add `--` after the package name, for example:

```sh
cargo run -p ex2_guessing_game -- --help
```
