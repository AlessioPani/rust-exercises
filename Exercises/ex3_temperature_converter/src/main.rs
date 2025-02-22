use colored_text::Colorize;
use std::io::{self, Write};

fn main() {
    println!("---- Welcome to Temperature Converter! ----\n");
    println!("-- Formulas --");
    println!("1. Fahrenheit -> Celsius: °C = (°F − 32) / 1,8");
    println!("2. Celsius -> Fahrenheit: °F = °C × 1,8 + 32");

    // Conversion selection.
    println!("\n=> Select the conversion:");
    println!("1. Fahrenheit -> Celsius");
    println!("2. Celsius -> Fahrenheit\n");
    print!("=> ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line!");

    let operation: u32 = match operation.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Insert a number!".red());
            return;
        }
    };

    // Run the appropriate function.
    match operation {
        1 => fahrenheit_to_celsius(),
        2 => celsius_to_fahrenheit(),
        _ => {
            println!("{}", "Only 1 or 2 are valid operation codes!".red());
            return;
        }
    };
}

// Function used to convert a Fahrenheit temperature in Celsius.
fn fahrenheit_to_celsius() {
    // Take the input from the user.
    print!(
        "{}",
        "\n=> Insert the Fahrenheit degree to be converted: ".yellow()
    );
    io::stdout().flush().expect("Failed to flush stdout");

    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Insert a number!".red());
            return;
        }
    };

    // Returns the required output.
    let celsius: f32 = (fahrenheit - 32.0) / 1.8;
    let output: String = format!("\n=> {fahrenheit}F° is equal to {celsius:.2}C°");
    println!("{}", output.green());
}

// Function used to convert a Celsius temperature in Fahrenheit .
fn celsius_to_fahrenheit() {
    // Take the input from the user.
    print!(
        "{}",
        "\n=> Insert the Celsius degree to be converted: ".yellow()
    );
    io::stdout().flush().expect("Failed to flush stdout");

    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f32 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Insert a number!".red());
            return;
        }
    };

    // Returns the required output.
    let fahrenheit: f32 = celsius * 1.8 + 32.0;
    let output: String = format!("\n=> {celsius}C° is equal to {fahrenheit:.2}F°");
    println!("{}", output.green());
}
