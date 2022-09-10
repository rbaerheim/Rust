use loop_unwrap::ToOption;
use std::{io, num::ParseIntError, process};

fn main() {
    println!("Hello, world!");
    happy_number();
}

fn check_string_parse_to_signed_int(user_input: String) -> Result<u32, ParseIntError> {
    match user_input.trim().parse::<u32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn integers_to_digits(digit: u32) -> Vec<u32> {
    let mut digit_copy: u32 = digit;
    let mut digit_vector: Vec<u32> = Vec::with_capacity(10);
    while digit_copy > 0 {
        let num_to_vec: u32 = integer_squared(digit_copy % 10);
        digit_copy = digit_copy / 10;
        digit_vector.push(num_to_vec);
    }
    digit_vector
}

fn integer_squared(number: u32) -> u32 {
    number.pow(2)
}

fn happy_number() -> u32 {
    println!(
        "Input a positive integer from 0 to 255 and I will tell you if it is a 'happy number': "
    );

    let mut num = loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
            println!("There was an error: {}", e);
            process::exit(1)
        });
        let num = loop_unwrap::unwrap_continue!(
            check_string_parse_to_signed_int(user_input),
            "Not a valid input, try again: "
        );
        break num;
    };

    while num != 1 {
        let digits_squared_vector_iter = integers_to_digits(num).iter();
        num = digits_squared_vector_iter.sum()
    }

    num
}
