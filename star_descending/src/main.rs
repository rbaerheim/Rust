use std::{io, process};

fn main() {
    let user_input: i32 = get_user_input();

    print_stars(user_input);
}

fn get_user_input() -> i32 {
    println!("Creates stars in console, input a number which indicates max stars: ");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("There was an error: {}", e);
        process::exit(1)
    });
    convert_user_string_to_int(user_input)
}

fn check_even() -> bool {
    println!("Do you want every other star to be printed? y/n");
    let mut even = String::new();
    io::stdin().read_line(&mut even).unwrap_or_else(|e| {
        println!("There was an error: {}", e);
        process::exit(1)
    });
    match even.as_str().trim() {
        "y" => true,
        "n" => false,
        _ => check_even(),
    }
}

fn convert_user_string_to_int(user_input: String) -> i32 {
    match user_input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            get_user_input()
        }
    }
}

fn print_stars(num: i32) {
    let mut even = 1;
    if check_even() {
        even = 2;
    }

    if num > 0 {
        for e in (0..num + 1).rev().step_by(even) {
            let stars = "*".repeat(e as usize);
            println!("{}", stars);
        }
    } else {
        let pos_num = num.abs();
        for e in (0..pos_num + 1).step_by(even) {
            let stars = "*".repeat(e as usize);
            println!("{}", stars);
        }
    }
}
