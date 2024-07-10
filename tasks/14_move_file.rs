use std::fs;
use std::io;

fn move_file(src: String, destination: String) -> io::Result<()> {
    fs::copy(&src, &destination)?;
    fs::remove_file(&src)?;
    Ok(())
}

fn main() {
    let mut src_input = String::new();
    let mut destination_input = String::new();

    println!("Enter file source path");
    io::stdin()
        .read_line(&mut src_input)
        .expect("failed to read path");

    println!("Enter file destination path");
    io::stdin()
        .read_line(&mut destination_input)
        .expect("failed to read path");

    let src: String = src_input.trim().to_string();
    let destination: String = destination_input.trim().to_string();

    if src.is_empty() || destination.is_empty() {
        eprintln!("Source path or Destination path cannot be empty");
    }

    if let Err(e) = move_file(src, destination) {
        eprintln!("Error moving file: {}", e);
    } else {
        println!("File moved successfully");
    }
}
