use std::io;

/// Main function: Entry point of the application
fn main() {
    println!("Welcome to the Rust Calculator App!");

    // Read the first number from the user
    println!("Please enter the first number: ");
    let num1: f64 = read_number();

    // Read the second number from the user
    println!("Please enter the second number: ");
    let num2: f64 = read_number();

    // Ask the user for the operation to perform
    println!("Choose an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read operation");

    // Perform the calculation based on the chosen operation
    let result = match operation.trim() {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Error: Invalid operation selected.");
            return; // Exit the program if the operation is invalid
        }
    };

    // Display the result of the calculation
    println!("The result is: {}", result);
}

/// Function to read a number from the user's input
/// Returns: The number as a `f64`
fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number entered")
}

/// Function to add two numbers
/// Parameters:
/// - `a`: First number as `f64`
/// - `b`: Second number as `f64`
/// Returns: The sum of `a` and `b`
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Function to subtract two numbers
/// Parameters:
/// - `a`: First number as `f64`
/// - `b`: Second number as `f64`
/// Returns: The difference between `a` and `b`
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Function to multiply two numbers
/// Parameters:
/// - `a`: First number as `f64`
/// - `b`: Second number as `f64`
/// Returns: The product of `a` and `b`
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Function to divide two numbers
/// Parameters:
/// - `a`: First number as `f64`
/// - `b`: Second number as `f64`
/// Returns: The quotient of `a` and `b`, or `0.0` if `b` is zero
fn divide(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Error: Division by zero is not allowed.");
        0.0
    }
}

