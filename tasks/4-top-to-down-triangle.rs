use std::io;

fn triangle (number: u64) -> String {
    let mut result = String::new();

    for i in 1..= number {
        for _ in 1..i {
            result.push('*');
        }
        result.push('\n');
    }
    result
}

fn main () {
    println!("Enter line number");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let number = input.trim().parse::<u64>().expect("Input not a number");

    let triangle = triangle(number);

    println!("{triangle}");
}


