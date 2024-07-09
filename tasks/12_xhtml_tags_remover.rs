use std::io;

fn main() {
    println!("Enter Xhtml");
    let mut xhtml_input = String::new();
    io::stdin()
        .read_line(&mut xhtml_input)
        .expect("failed to read input");

    let xhtml: String = xhtml_input.trim().replace("\n", " ").to_string();
    parse(xhtml);
}

fn parse(xhtml: String) {

    let chars: Vec<char> = xhtml.chars().collect();

    let mut result = String::new();
    let mut _inside_tag: bool = false;

    for char in chars  {

        if char == '<' {
            _inside_tag = true;
        } else if char == '>' {
            _inside_tag = false;

        } else if !_inside_tag && char != '<' && char != '>' {
            result.push(char);
        }
    }
    println!("\nYour parsed xhtml:\n\n{result}");
}

