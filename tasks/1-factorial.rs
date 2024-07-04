fn main() {

    println!("factorial answer: {}", factorial(5));
}

fn factorial(number: u64) ->u64 {
    let mut result : u64 = 1;
    for i in 1..= number {
        result *= i;
    }
    return result;
}
