use std::io;

fn perform_calculation(first_number: i64, second_number: i64, operator: char ) -> i64 {

    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Not a valid operator")
    }
}

fn main() {
    println!("Enter your first number");
    let mut first_number_input = String::new();

    io::stdin()
        .read_line(&mut first_number_input)
        .expect("failed to read input");

    println!("Enter your operator sign");
    let mut operator_input = String::new();

    io::stdin()
        .read_line(&mut operator_input)
        .expect("failed to read input");


    println!("Enter your second number");
    let mut second_number_input = String::new();

    io::stdin()
        .read_line(&mut second_number_input)
        .expect("failed to read input");

    let first_number = first_number_input.trim().parse::<i64>().expect("Input is not number");
    let second_number = second_number_input.trim().parse::<i64>().expect("Input is not number");
    let operator = operator_input.trim().parse::<char>().expect("Input is not number");
    let total: i64 = perform_calculation(first_number, second_number, operator);
    println!("Your Answer is: {total}");
}
