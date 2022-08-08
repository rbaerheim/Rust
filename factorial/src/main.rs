// This program returns the factorial of user input

use std::io;
use std::io::Write;
use std::process;
fn main() {
    // Using the flush() method from use std::io::Write to get to write input on same line as question.
    print!("Enter a number > 0: ");
    io::stdout().flush().unwrap();

    // Creating a mutable empty string for the user to write to
    let mut user_input: String = String::new();

    // This reads the input to the string
    io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });

    // Parses the string to an integer
    let user_input_int: i32 = user_input.trim().parse().unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1)
    });

    // Using the function to return the factorial
    let total: i32 = factorial(user_input_int);

    // Printing the answer to the terminal
    println!("The factorial of {} is {}", user_input_int, total);
}

fn factorial(input: i32) -> i32 {
    let mut total: i32 = 1;

    if input == 0 {
        return 0;
    }

    // Iterating over the input up to and including the input.
    // Returning the total
    for num in 1..=input {
        total *= num;
    }

    total
}
