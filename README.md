# Rust Exercises

A repository of exercises for learning Rust. The roadmap starts with the programs already in the repository and continues with exercises that together cover the concepts in *The Rust Programming Language* (the Rust Book) and *Rust By Example* (RBE).

## Cargo workspace

The exercises under `Exercises/` are members of a single Cargo workspace. Run these commands from the repository root:

```sh
# Build every exercise
cargo build --workspace

# Run an exercise by package name
cargo run -p ex1_hello_world

# Check the code without producing an executable
cargo check --workspace
```

To pass arguments to a program, add `--` after the package name, for example: `cargo run -p ex2_guessing_game -- --help`.

## Exercise roadmap

Exercises 1–5 already exist. The remaining exercises are proposals, listed in a suggested order. A project can be split into multiple binaries, modules, or crates when appropriate.

### Fundamentals and control flow

1. `ex1_hello_world` — printing, comments, compilation, and a first Cargo project. *(existing)*
2. `ex2_guessing_game` — input, parsing, mutability, shadowing, `match`, `loop`, `Result`, and dependencies. *(existing)*
3. `ex3_temperature_converter` — functions, numeric types, input/output, and conditions. *(existing)*
4. `ex4_fibonacci_nth` — functions, recursion, integers, and type limits. *(existing)*
5. `ex5_christmas_carol` — arrays, slices, tuples, iteration, ranges, and formatting. *(existing)*
6. **Command-line calculator** (`ex6_cli_calculator`) — a menu, operators, expressions, `if`, `match`, loops, an enum for operations, and invalid input handling. Add explicit type conversions and functions with parameters and return values.
7. **Number analyzer** (`ex7_number_lab`) — calculate statistics for a sequence, using primitive types, casts, tuples, arrays/slices, loops, functions, and handling overflow or empty input.

### Ownership, structs, enums, and collections

8. **Address book** (`ex8_address_book`) — `struct`, methods and associated functions, `String` versus `&str`, ownership, borrowing, mutable references, and slices. Support searching and editing without unnecessary copies.
9. **Library catalog** (`ex9_library`) — data-bearing enums, `Option`, `match`, `if let`/`let else`, `Vec`, and `HashMap`; implement borrowing and returning items while preserving explicit invariants.
10. **Expression parser** (`ex10_expression_parser`) — pattern matching, recursive enums, `Box`, stacks, `while let`, destructuring, match guards, `@` bindings, and conversions between strings and numeric types.
11. **Persistent address book** (`ex11_persistent_contacts`) — file reading and writing, `Path`/`PathBuf`, `File`, `BufReader`/`BufWriter`, `?`, propagated errors, and text formats. Handle and document missing or malformed files.

### Modules, crates, generics, and traits

12. **CSV expense tracker** (`ex12_expense_tracker`) — split the model, parsing, and interface into modules; use visibility (`pub`), paths, `use`, nested modules, `String`, `Vec`, iterators, and `Result`. Add a reusable library and a binary.
13. **Statistics library** (`ex13_statistics`) — generic functions and structs, trait bounds, trait implementations, standard traits (`Display`, `Debug`, `Clone`, `PartialEq`), and associated types. Add a binary with usage examples.
14. **Configurable search** (`ex14_search`) — traits for search strategies, generics and trait objects (`dyn`), closures, higher-order functions, iterator adapters (`map`, `filter`, `fold`), and explicit lifetimes for results borrowed from the input text.
15. **Mini collection and shared ownership** (`ex15_collection`) — implement a custom iterator and a generic type with `IntoIterator`; explore `Drop`, `Deref`, and operator overloading where appropriate. Add focused examples using `Box`, `Rc`, `Arc`, `Cell`, `RefCell`, and `Cow`, and explain when heap allocation, reference counting, copy-on-write, or interior mutability is useful.
16. **Advanced types and APIs lab** (`ex16_advanced_types`) — newtypes, type aliases, the `!` type, functions and closures as parameters or values, function pointers, associated types, supertraits, conditional implementations, UFCS syntax, and operator overloading. Compare each API choice with a simpler alternative.

### Errors, tests, and tools

17. **File search CLI** (`ex17_mini_grep`) — command-line arguments, configuration, the filesystem, custom error enums, `Result`, `?`, `Box<dyn Error>`, stdout/stderr, and usage documentation.
18. **Address book test suite** — unit, integration, and documentation tests; `assert!`/`assert_eq!`, ignored and conditional tests. Cover errors and edge cases as well.
19. **Configuration and diagnostics** (`ex19_config`) — environment variables, `Option` and `Result`, useful error messages, basic logging, and attributes (`derive`, `cfg`, `allow`, `test`, `should_panic`). Add examples and rustdoc comments.
20. **Macros for tests and output** (`ex20_macros`) — implement declarative macros with `macro_rules!`, repetition, and patterns; compare macros with functions. As a separate workspace crate, implement a small derive procedural macro and use it from an application crate.

### Concurrency and advanced topics

21. **Thread pool** (`ex21_thread_pool`) — build a reusable pool with worker threads, a job queue, channels, `Send`/`Sync`, shared state, and graceful shutdown. Test task execution and shutdown behavior.
22. **Multithreaded web server** (`ex22_web_server`) — build on the pool to accept TCP connections, parse a small subset of HTTP requests, return correct status lines and headers, and shut down cleanly. Separate request parsing, response generation, and server control; test each part. An async rewrite is an optional comparison, not a substitute for the thread-pool exercise.
23. **Lifetimes lab** (`ex23_lifetimes`) — structs containing references, lifetimes in functions and methods, elision, the `'static` lifetime, trait bounds, and borrow checker limits. Include compilable examples and short explanations.
24. **Unsafe and FFI lab** (`ex24_unsafe_lab`) — raw pointers, unsafe functions, mutable statics, unsafe trait implementations, union fields, and a call to a small C function. Encapsulate one operation behind a safe API and state its safety invariants. Keep each unsafe example isolated and explain why its preconditions hold.
25. **Rust compatibility lab** (`ex25_compatibility`) — set an MSRV and edition policy, inspect compiler and dependency requirements, use conditional compilation with `cfg`, and make a small API change while documenting compatibility implications. Check the project with the selected toolchain.
26. **Release builds and benchmarks** (`ex26_release_bench`) — Cargo debug/release profiles, features and dependencies, workspace organization, `cargo fmt`, `cargo clippy`, rustdoc examples, and benchmarks using a dedicated tool. Compare two implementations and record the platform and method so results can be interpreted.
27. **System utility** (`ex27_system_tools`) — explore `std::env`, `std::process`, paths, CLI arguments, directories and files, exit codes, and child processes. Build a small utility that searches for files and summarizes their sizes.



## Useful commands

```sh
cargo build --workspace
cargo run -p ex2_guessing_game
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo doc --workspace --open
```
