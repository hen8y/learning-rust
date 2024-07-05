use std::io;

fn factorial(number: u64) ->u64 {
    let mut result : u64 = 1;
    for i in 1..= number {
        result *= i;
    }
    return result;
}

fn main() {
    println!("Enter a number");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");

    let number: f64 = input.trim().parse::<u64>().expect("Input is not a number");
    let factorial: f64 = factorial(number);

    println!("factorial answer: {factorial}");
}
