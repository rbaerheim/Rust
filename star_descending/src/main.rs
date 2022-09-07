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
    if num > 0 {
        for e in (0..num + 1).rev() {
            let stars = "*".repeat(e as usize);
            println!("{}", stars);
        }
    } else {
        let pos_num = num.abs();
        for e in 0..pos_num + 1 {
            let stars = "*".repeat(e as usize);
            println!("{}", stars);
        }
    }
}
