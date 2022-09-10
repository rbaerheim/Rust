use loop_unwrap::unwrap_continue;
use std::{io, num::ParseIntError, process};

fn main() {
    println!("Hello, world!");
    happy_number();
}

#[allow(unused)]
fn check_string_parse_to_signed_int(user_input: String) -> Result<i32, ParseIntError> {
    match user_input.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn happy_number() -> bool {
    println!(
        "Input a positive integer from 0 to 255 and I will tell you if it is a 'happy number': "
    );
    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
            println!("There was an error: {}", e);
            process::exit(1)
        });

        let num = unwrap_continue!(check_string_parse_to_signed_int(user_input));
    }
}
