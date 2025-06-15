use std::io;
use std::str::FromStr;

fn main() {
    let k: f64 = read_input("Enter k:");
    let x: f64 = read_input("Enter x:");
    let n: f64 = read_input("Enter n:");
    let acceleration = (k - x) * 60.0 / n;
    println!("acceleration is {}", acceleration);
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
