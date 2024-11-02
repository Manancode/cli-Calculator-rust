//Understanding the Code
// The program starts by collecting the command line arguments into a vector named args. It expects three arguments: two numbers and an operator.

// It then checks if the correct number of arguments are provided. If not, it displays a usage message and exits.

// The numbers are parsed from the command line arguments and stored in num1 and num2 as floating-point numbers to allow for decimal calculations. The program exits if the parsing fails, indicating the input was not a valid number.

// The match statement is used to determine which arithmetic operation to perform based on the operator argument. It outputs the result of the operation or exits if an invalid operator is provided.

use std::env;    // For command line arguments
use std::process;    // For exit handling

fn main() {
    // Collect command line arguments into a vector
    // args[0] is the program name
    // args[1] is the first number
    // args[2] is the operator
    // args[3] is the second number
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the correct number of arguments
    // We need exactly 4 arguments (program name + 2 numbers + operator)
    if args.len() != 4 {
        // Print error message to stderr
        eprintln!("Usage: calc [num1] [operator] [num2]");
        eprintln!("Example: calc 10 + 5");
        // Exit with error code 1
        process::exit(1);
    }

    // Parse the first number
    // expect() will crash with the given message if parsing fails
    let num1: f64 = args[1]
        .parse()
        .expect("First argument is not a valid number");

    // Get the operator as a reference
    let operator = &args[2];

    // Parse the second number
    let num2: f64 = args[3]
        .parse()
        .expect("Third argument is not a valid number");

    // Match the operator and perform calculation
    match operator.as_str() {
        // Addition
        "+" => println!("Result: {}", num1 + num2),
        // Subtraction
        "-" => println!("Result: {}", num1 - num2),
        // Multiplication
        "*" => println!("Result: {}", num1 * num2),
        // Division
        "/" => {
            // Check for division by zero
            if num2 == 0.0 {
                eprintln!("Error: Division by zero!");
                process::exit(1);
            }
            println!("Result: {}", num1 / num2)
        },
        // Invalid operator
        _ => {
            eprintln!("Invalid operator. Please use +, -, *, or /.");
            eprintln!("Example: calc 10 + 5");
            process::exit(1);
        }
    }
}