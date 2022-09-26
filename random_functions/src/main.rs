use loop_unwrap::ToOption;
use std::{collections::HashMap, io, num::ParseIntError, process};

fn main() {
    let is_happy = happy_number();

    println!("{}", is_happy);
}

fn check_string_can_be_parsed_to_u32(user_input: &String) -> Result<u32, ParseIntError> {
    match user_input.trim().parse::<u32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn integers_split_to_individual_digits(digit: u32) -> Vec<u32> {
    let mut digit_copy: u32 = digit;
    let mut digit_hashmap: Vec<u32> = Vec::with_capacity(10);
    while digit_copy > 0 {
        let num_to_vec: u32 = (digit_copy % 10).pow(2);
        digit_copy = digit_copy / 10;
        digit_hashmap.push(num_to_vec);
    }
    digit_hashmap
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    user_input = loop {
        io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
            println!("There was an error: {}", e);
            process::exit(1)
        });
        break user_input;
    };
    user_input
}

fn prime_factorization() -> String {
    let (mut num, user_input) = loop {
        println!("Input a number between 0 and 4294967295 and I will perform a prime factorization of that number: ");
        let user_input = get_user_input();

        let num = loop_unwrap::unwrap_continue!(
            check_string_can_be_parsed_to_u32(&user_input),
            "Not a valid input, try again: "
        );
        break (num, user_input);
    };
    let mut divisor: u32 = 2;
    loop {
        if num % divisor == 0 {
            break;
        }
    }
    user_input
}

fn happy_number() -> String {
    let (mut num, user_input) = loop {
        println!("Input a number between 0 and 4294967295 and I will tell you if it is a 'happy number': ");
        let user_input = get_user_input();

        let num = loop_unwrap::unwrap_continue!(
            check_string_can_be_parsed_to_u32(&user_input),
            "Not a valid input, try again: "
        );
        break (num, user_input);
    };

    let mut control_hashmap = HashMap::new();

    loop {
        num = integers_split_to_individual_digits(num).iter().sum();
        if num == 1 {
            return format!("{} is a happy number!", user_input.trim());
        }
        if control_hashmap.contains_key(&num) {
            return format!("{} is not a happy number..", user_input.trim());
        }
        control_hashmap.insert(num, false);
    }
}
