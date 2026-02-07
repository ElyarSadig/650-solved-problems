use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let s = input.split_whitespace().next().unwrap_or("");

    print!("Enter a string: ");
    for ch in s.chars() {
        print!("{} ", ch);
    }
}