use std::io::{self, Write};

fn main() {
    println!("This application calculates the n-th number of the Fibonacci's series.\n");

    // Get the user input.
    print!("Insert the n-th Fibonacci's number you want to generate: ");
    io::stdout().flush().expect("Failed to flush stout");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: usize = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Insert a positive number!");
            return;
        }
    };

    // Get the requested Fibonacci's number.
    let result = fibonacci(number);
    println!("\n=> The {number}th number of Fibonacci is {result}.");
}

// Function that calculates the n-th number of the Fibonacci's series
// using recursion.
fn fibonacci(nth: usize) -> usize {
    if nth < 2 {
        1
    } else {
        fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}
