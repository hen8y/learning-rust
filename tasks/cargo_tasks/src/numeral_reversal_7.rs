use std::io;

fn reversed_string(number: u64) -> String {
    let result = number.to_string().chars().rev().collect();
    result
}


pub fn reversal() {
    println!("Enter a number");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let number = input.trim().parse::<u64>().expect("input not a number");

    println!("Your reserved number: {}", reversed_string(number));
}


