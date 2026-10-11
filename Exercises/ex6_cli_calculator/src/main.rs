use std::io;

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

    loop {
        operation.clear();

        println!("\nSelect your operation: ");

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation: u8 = match operation.trim().parse() {
            Ok(0) => {
                println!("Goodbye");
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

        println!("The result is {result}");
    }
}

fn print_menu() {
    println!("Welcome to the Command-Line calculator");
    println!("1: Sum ");
    println!("2: Substraction");
    println!("3: Multiplication");
    println!("4: Division");
    println!("5: Square N");
    println!("6: Power N");
    println!("0: Quit");
}

fn add() -> f64 {
    let a: f64 = read_number("\nInsert the first number: ");
    let b: f64 = read_number("\nInsert the second number: ");

    a + b
}

fn substract() -> f64 {
    let a: f64 = read_number("\nInsert the first number: ");
    let b: f64 = read_number("\nInsert the second number: ");

    a - b
}

fn multiply() -> f64 {
    let a: f64 = read_number("\nInsert the first number: ");
    let b: f64 = read_number("\nInsert the second number: ");

    a * b
}

fn divide() -> f64 {
    let dividend: f64 = read_number("\nInsert the first number: ");
    let mut divisor: f64 = read_number("\nInsert the second number: ");

    while divisor == 0 as f64 {
        println!("Divisor cannot be 0: insert another number");
        divisor = read_number("\nInsert the second number:");
    }

    dividend / divisor
}

fn square_n() -> f64 {
    let mut base: f64 = read_number("\nInsert the base: ");
    let mut exponent: f64 = read_number("\nInsert the root degree: ");

    while base < 0 as f64 {
        println!("Base cannot be negative for real values");
        base = read_number("Insert the base");
    }

    while exponent <= 0 as f64 {
        println!("Root degree cannot be negative or zero");
        exponent = read_number("\nInsert the root degree: ");
    }

    base.powf(1.0 / exponent)
}

fn power_n() -> f64 {
    let base: f64 = read_number("\nInsert the base: ");
    let mut exponent: f64 = read_number("\nInsert the exponent: ");

    while exponent < 0 as f64 {
        println!("Exponent cannot be negative");
        exponent = read_number("\nInsert the exponent: ");
    }

    base.powf(exponent)
}

fn read_number(message: &str) -> f64 {
    println!("{}", message);
    let mut number: String = String::new();

    loop {
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
