use std::{io, process};

#[derive(Debug)]
struct Currency<'a> {
    name: &'a str,
    code: &'a str,
    value_in_nok: f32,
}

impl Currency<'_> {
    fn get_code<'a>(&'a self) -> &'a str {
        return self.code;
    }
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

    println!(
        "This is a currency converter which converts EUR or USD to NOK, input which currency you want(Currently only supports USD and EUR): "
    );

    let code_exists = check_code_exists(&currency_vector);

    println!(
        "Currency code exists: {}, input amount to get converted:",
        code_exists.trim()
    );

    io::stdin()
        .read_line(&mut currency_value_input)
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });

    let converted_value = convert(
        code_exists.trim().to_string().to_uppercase(),
        &currency_value_input,
        &currency_vector,
    );

    if converted_value.0 == 0.0 {
        println!("{}", converted_value.1)
    } else {
        println!(
            "{} {}(s) is currently {} NOK.",
            currency_value_input.trim(),
            converted_value.1,
            converted_value.0
        );
    }
}

fn check_code_exists(currencies_available: &[Currency]) -> String {
    loop {
        let mut currency_code_input: String = String::new();
        io::stdin()
            .read_line(&mut currency_code_input)
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                process::exit(1);
            });
        let check = check_if_convertable(currency_code_input, &currencies_available);
        if check.0 {
            return check.1;
        }
        if check.1.trim().to_lowercase() == "Q".to_lowercase() {
            println!("Exiting...");
            process::exit(1);
        }
        println!(
            "'{}' did not match any of our currency codes, try again or enter Q to exit.",
            check.1.trim()
        );
    }
}

fn check_if_convertable<'a>(
    currency_code: String,
    currencies_available: &'a [Currency],
) -> (bool, String) {
    let currencies_available_iter = currencies_available.iter();
    for currency in currencies_available_iter {
        if currency.get_code() == currency_code.trim() {
            return (true, currency_code.to_string());
        }
    }
    (false, currency_code.to_string())
}

fn convert(in_code: String, in_value: &String, currencies_available: &[Currency]) -> (f32, String) {
    let input_value_float: f32 = in_value.trim().parse().unwrap();
    let currencies_iter = currencies_available.iter();
    for currency in currencies_iter {
        if in_code == currency.code {
            return (
                (currency.value_in_nok * input_value_float),
                currency.name.to_string(),
            );
        };
    }

    (0.0, String::from("An error happened, try again!"))
}
