use std::io;
use rand::Rng;
use rand::thread_rng;

fn generate_number() -> u64 {
    let result: u64 = thread_rng().gen_range(1 ..= 100);

    result
}


pub fn guess() {
    println!("Enter number 1 - 100");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let number = input.trim().parse::<u64>().expect("Input not a number");

    let guess: u64 = generate_number();
    let result: &str;

    if number == guess {
        result = "You win!";
    } else if number > guess {
        result = "Too high";
    } else {
        result = "Too low";
    }

    println!("{result} {guess}");
}

