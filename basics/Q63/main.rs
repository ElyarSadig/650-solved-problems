use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    print!("Enter a char: ");
    io::stdout().flush().ok();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let ch = input.chars().find(|c| *c != '\n' && *c != '\r').expect("No character provided");
    let code = ch as u32;

    println!("{}", code);
}