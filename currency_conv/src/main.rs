use std::{io, process};

#[derive(Debug)]
struct Currency<'a> {
    name: &'a str,
    code: &'a str,
    value_in_nok: f32,
}

const USD: Currency = Currency {
    name: "American Dollar",
    code: "USD",
    value_in_nok: 9.6682,
};

const NOK: Currency = Currency {
    name: "Norwegian Kroner",
    code: "NOK",
    value_in_nok: 1.0,
};

const EUR: Currency = Currency {
    name: "Euro",
    code: "EUR",
    value_in_nok: 9.9118,
};

fn main() {
    let currency_vector = vec![NOK, USD, EUR];
    let mut currency_value_input: String = String::new();
    let mut currency_code_input: String = String::new();

    println!(
        "This is a currency converter which converts EUR or USD to NOK, input which currency you want(Currently only supports USD and EUR): "
    );

    io::stdin()
        .read_line(&mut currency_code_input)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });

    println!("Input amount to get converted:");

    io::stdin()
        .read_line(&mut currency_value_input)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });

    let input_to_float: f32 = currency_value_input.trim().parse().unwrap();

    let converted_value = convert(
        currency_code_input.trim().to_string().to_uppercase(),
        input_to_float,
        &currency_vector,
    );

    if converted_value.0 == 0.0 {
        println!("{}", converted_value.1)
    } else {
        println!(
            "{} {} is currently {} NOK",
            input_to_float, converted_value.1, converted_value.0
        );
    }
}

fn convert(in_code: String, in_value: f32, currencies_available: &[Currency]) -> (f32, String) {
    let currencies_iter = currencies_available.iter();
    for currency in currencies_iter {
        if in_code == currency.code {
            return (
                (currency.value_in_nok * in_value),
                currency.name.to_string(),
            );
        };
    }

    (0.0, String::from("An error happened, try again!"))
}
