use colored_text::Colorize;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to the Guessing Game!\n");

    // Generate a random number.
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        // Read user input and convert it into an integer.
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "\nInsert a number!\n".red());
                continue;
            }
        };

        println!("\nYou have guessed: {guess}.");

        // Compare the numbers.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!\n".yellow()),
            Ordering::Greater => println!("{}", "Too big!\n".yellow()),
            Ordering::Equal => {
                println!("{}", "You won!\n".green());
                break;
            }
        }
    }
}
