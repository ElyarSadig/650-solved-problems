use std::io;
use std::str::FromStr;

fn main() {
    let height: f64 = read_input("Enter Height:");
    let base: f64 = read_input("Enter Base:");
    let area = height * base * 0.5;
    println!("Area is {}", area);
}

fn read_input<T: FromStr>(prompt: &str) -> T {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<T>() {
        Ok(value) => return value,
        Err(_e) => panic!("Invalid input: please try again."),
    }
}
