use std::{io, process};

#[derive(Debug)]
struct Currency {
    name: String,
    code: String,
    value_in_nok: f32,
}

fn main() {
    let curr1: Currency = Currency {
        name: String::from("American Dollar"),
        code: (String::from("USD")),
        value_in_nok: 9.6682,
    };

    let curr2: Currency = Currency {
        name: String::from("Norwegian Kroner"),
        code: (String::from("NOK")),
        value_in_nok: 1.0,
    };

    let curr3: Currency = Currency {
        name: String::from("Euro"),
        code: (String::from("EUR")),
        value_in_nok: 9.9118,
    };

    let currency_vector = vec![curr1, curr2, curr3];

    println!("This is a currency converter which converts Norwegian Kroner to other currencies, please input the currency code you want: ");
    let mut currency_code_input: String = String::new();
    let mut currency_value_input: String = String::new();

    io::stdin()
        .read_line(&mut currency_code_input)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });

    io::stdin()
        .read_line(&mut currency_value_input)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });

    let input_to_float: f32 = currency_value_input.trim().parse().unwrap();

    let converted_value = convert(
        currency_code_input.trim().to_string(),
        input_to_float,
        &currency_vector,
    );

    println!("Converted value: {}", converted_value);
}

fn convert(in_code: String, in_value: f32, currencies_available: &[Currency]) -> f32 {
    let currencies_iter = currencies_available.iter();
    for currency in currencies_iter {
        if in_code == currency.code {
            return currency.value_in_nok * in_value;
        };
    }

    0.0
}
