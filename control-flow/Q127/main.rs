use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a string:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }

    let count = input.chars().count();
    println!("Count is {}", count);
}