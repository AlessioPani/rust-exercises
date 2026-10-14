use std::io::{self, Write};

// Represents the operations available through the command-line interface.
enum Operation {
    Add,
    Substract,
    Multiply,
    Divide,
    SquareN,
    PowerN,
}

impl TryFrom<u8> for Operation {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Substract),
            3 => Ok(Self::Multiply),
            4 => Ok(Self::Divide),
            5 => Ok(Self::SquareN),
            6 => Ok(Self::PowerN),
            _ => Err(()),
        }
    }
}

fn main() {
    print_menu();

    let mut operation: String = String::new();

    // Process operations until the user selects the quit option.
    loop {
        operation.clear();

        print!("\n=> Select your operation: ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation: u8 = match operation.trim().parse() {
            Ok(0) => {
                println!("\nGoodbye!");
                // Exit the application after the user selects the quit option.
                return;
            }
            Ok(code) => code,
            _ => {
                println!("Insert a valid op code: please try again!");
                continue;
            }
        };

        let result = match Operation::try_from(operation) {
            Ok(Operation::Add) => add(),
            Ok(Operation::Substract) => substract(),
            Ok(Operation::Multiply) => multiply(),
            Ok(Operation::Divide) => divide(),
            Ok(Operation::SquareN) => square_n(),
            Ok(Operation::PowerN) => power_n(),
            Err(_) => {
                println!("Invalid operation code: try again!");
                continue;
            }
        };

        println!("\n=> The result is {result}");
    }
}

/// Displays the available operations and their selection codes.
fn print_menu() {
    println!("\n|-----------------------------------------------|");
    println!("| Welcome to the Command-Line calculator        |");
    println!("|-----------------------------------------------|");
    println!("| 1: Sum                                        |");
    println!("| 2: Substraction                               |");
    println!("| 3: Multiplication                             |");
    println!("| 4: Division                                   |");
    println!("| 5: Square N                                   |");
    println!("| 6: Power N                                    |");
    println!("|-----------------------------------------------|");
    println!("| 0: Quit                                       |");
    println!("|-----------------------------------------------|");
}

/// Reads two values and returns their sum.
fn add() -> f64 {
    let a: f64 = read_number("\nInsert the first number: ");
    let b: f64 = read_number("Insert the second number: ");

    a + b
}

/// Reads two values and returns their difference.
fn substract() -> f64 {
    let a: f64 = read_number("\nInsert the first number: ");
    let b: f64 = read_number("Insert the second number: ");

    a - b
}

/// Reads two values and returns their product.
fn multiply() -> f64 {
    let a: f64 = read_number("\nInsert the first number: ");
    let b: f64 = read_number("Insert the second number: ");

    a * b
}

/// Reads a dividend and a non-zero divisor, then returns their quotient.
fn divide() -> f64 {
    let dividend: f64 = read_number("\nInsert the dividend: ");
    let mut divisor: f64 = read_number("\nInsert the divisor: ");

    // Require a non-zero divisor before performing the division.
    while divisor == 0 as f64 {
        println!("Divisor cannot be 0: insert another number");
        divisor = read_number("\nInsert the divisor: ");
    }

    dividend / divisor
}

/// Reads a non-negative base and positive root degree, then returns the nth root.
fn square_n() -> f64 {
    let mut base: f64 = read_number("\nInsert the base: ");
    let mut exponent: f64 = read_number("Insert the root degree: ");

    // Validate the inputs to ensure the result is a real-valued root.
    while base < 0 as f64 {
        println!("Base cannot be negative for real values");
        base = read_number("Insert the base: ");
    }

    while exponent <= 0 as f64 {
        println!("Root degree cannot be negative or zero");
        exponent = read_number("\nInsert the root degree: ");
    }

    base.powf(1.0 / exponent)
}

/// Reads a base and non-negative exponent, then returns the base raised to that power.
fn power_n() -> f64 {
    let base: f64 = read_number("\nInsert the base: ");
    let mut exponent: f64 = read_number("Insert the exponent: ");

    // Require a non-negative exponent for this operation.
    while exponent < 0 as f64 {
        println!("Exponent cannot be negative");
        exponent = read_number("\nInsert the exponent: ");
    }

    base.powf(exponent)
}

/// Prompts for input and returns it as a floating-point value.
///
/// Repeats the prompt after invalid numeric input.
fn read_number(message: &str) -> f64 {
    let mut number: String = String::new();

    // Continue prompting until the input parses as a floating-point value.
    loop {
        print!("{}", message);
        io::stdout().flush().expect("Failed to flush stdout");

        number.clear();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                println!("Insert a valid value: please try again!");
                continue;
            }
        };
    }
}
