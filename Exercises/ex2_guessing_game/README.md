# Chapter 2



## Assessment

- Guessing game with a random-generated number and a user input to be compared.



## Variables

- By default, variables are immutable

- To define a variable mutable, use the "mut" keyword

  ```rust
  let mut apples = 5;
  ```

  

##  Standard Library

- String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text

- To define a new String variable, use its associated function new() (associated function => function implemented on a type)

  ```rust
  let mut guess = String::new()
  ```



## Cargo - Dependencies

- To add a dependency (a crate), edit Cargo.toml accordingly

- To update crates, run

  ```bash
  cargo update
  ```

- Documentation of used crates and functions

  ```bash
  cargo doc --open
  ```

  