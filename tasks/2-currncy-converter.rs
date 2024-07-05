use std::io;

const NGN_TO_USD_RATE: f64 = 0.00064;
const NGN_TO_EURO_RATE: f64 = 0.00059;

fn convert (currency: &str, amount: f64) ->f64 {
    let converted_amount: f64;
    if currency == "USD" {
        converted_amount = amount * NGN_TO_USD_RATE;
    } else {
        converted_amount = amount * NGN_TO_EURO_RATE;
    }

    converted_amount
}

fn main () {
    let mut currency_input = String::new();
    let mut amount_input = String::new();
    println!("Enter currency (USD or EURO):");
    io::stdin()
        .read_line(&mut currency_input)
        .expect("failed to get input");

    println!("Enter Amount:");
    io::stdin()
        .read_line(&mut amount_input)
        .expect("failed to get input");


    let amount: f64 = amount_input.trim().parse().expect("Input is not a number");
    let currency: &str = currency_input.trim();


    let result: f64 = convert(currency, amount);
    println!("Your converted value: {result}");
}

