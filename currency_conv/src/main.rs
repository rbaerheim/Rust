use std::{io, num::ParseFloatError, process};

#[allow(unused)]
#[derive(Debug)]
struct Currency<'a> {
    name: &'a str,
    code: &'a str,
    exchange_rate: f32,
}

impl Currency<'_> {
    fn get_code<'a>(&'a self) -> &'a str {
        return self.code;
    }

    fn get_exchange_rate<'a>(&'a self) -> &'a f32 {
        return &self.exchange_rate;
    }
}

const USD: Currency = Currency {
    name: "American Dollar",
    code: "USD",
    exchange_rate: 1.00,
};

const NOK: Currency = Currency {
    name: "Norwegian Kroner",
    code: "NOK",
    exchange_rate: 9.8081,
};

const GBP: Currency = Currency {
    name: "British Pound",
    code: "GBP",
    exchange_rate: 0.84575,
};

const JPY: Currency = Currency {
    name: "Japanese Yen",
    code: "JPY",
    exchange_rate: 136.88,
};

const BRL: Currency = Currency {
    name: "Brazilian Real",
    code: "BRL",
    exchange_rate: 5.1911,
};

#[allow(unused)]
fn main() {
    let currency_vector = vec![NOK, USD, GBP, JPY, BRL];

    println!(
        "This is a currency converter which converts EUR or USD to NOK, input which currency you want(Currently only supports USD and EUR): "
    );

    let conversion_vector = check_code_exists(&currency_vector);

    println!(
        "You have chosen to convert from {} to {}, please input the value to be converted:",
        conversion_vector[0].0.trim(),
        conversion_vector[1].0.trim()
    );

    let converted_value = convert(conversion_vector[0].1, conversion_vector[1].1);
    println!(
        "{} {} is {} {}",
        converted_value.0,
        conversion_vector[0].0.trim(),
        converted_value.1,
        conversion_vector[1].0.trim()
    );
}

fn convert(from_exhange_rate: f32, to_exhange_rate: f32) -> (f32, f32) {
    let value = loop {
        match get_input_value() {
            Ok(value) => break value,
            Err(e) => println!("There was an error: '{}'. Please insert a valid number", e),
        }
    };
    (value, (to_exhange_rate / from_exhange_rate) * value)
}

fn get_input_value() -> Result<f32, ParseFloatError> {
    let value = loop {
        let mut value_to_convert: String = String::new();
        io::stdin()
            .read_line(&mut value_to_convert)
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                process::exit(1);
            });

        let value = value_to_convert.trim().parse::<f32>()?;

        if value > 0.0 {
            break value;
        };
        println!("The number to be converted must be positive.")
    };

    Ok(value)
}

fn check_code_exists(currencies_available: &[Currency]) -> Vec<(String, f32)> {
    let mut conversion_vector = Vec::new();
    loop {
        let mut currency_code_input: String = String::new();
        io::stdin()
            .read_line(&mut currency_code_input)
            .unwrap_or_else(|e| {
                println!("Error: {}", e);
                process::exit(1);
            });
        let check = check_if_convertable(currency_code_input, &currencies_available);
        if check.1.trim().to_lowercase() == "Q".to_lowercase() {
            println!("Exiting...");
            process::exit(1);
        }
        if check.0 && conversion_vector.len() < 1 {
            conversion_vector.push((check.1, check.2));
            println!(
                "You choose to convert from {}, what do you want it converted to?",
                conversion_vector[0].0.trim()
            );
        } else if check.0 {
            match check.1 {
                _ if check.1 == conversion_vector[0].0 => println!("You tried to convert {} to {}, these are the same, please enter another currency to convert to.", conversion_vector[0].0.trim(), check.1.trim()),
                _ => conversion_vector.push((check.1, check.2)),
            }
            if conversion_vector.len() == 2 {
                return conversion_vector;
            }
        } else {
            println!(
                "'{}' is not supported or do not exist, try another code or enter Q to exit.",
                check.1.trim()
            );
        }
    }
}

fn check_if_convertable<'a>(
    currency_code: String,
    currencies_available: &'a [Currency],
) -> (bool, String, f32) {
    let currencies_available_iter = currencies_available.iter();
    for currency in currencies_available_iter {
        if currency.get_code() == currency_code.trim() {
            return (
                true,
                currency_code.to_string(),
                *currency.get_exchange_rate(),
            );
        }
    }
    (false, currency_code.to_string(), 0.0)
}
