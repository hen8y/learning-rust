use rand::Rng;
use rand::thread_rng;


fn first() -> String {
    let result: &str = "first function";
    result.to_string()
}

fn second() -> String {
    let result: &str = "second function";
    result.to_string()
}

fn third() -> String {
    let result: &str = "third function";
    result.to_string()
}


fn main() {
    let number: u64 = thread_rng().gen_range(1 ..= 3);

    match number {
        1 => println!("{}", first()),
        2 => println!("{}", second()),
        3 => println!("{}", third()),
        _ => unreachable!()
    }
}
