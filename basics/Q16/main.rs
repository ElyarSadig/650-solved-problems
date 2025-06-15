use std::io;
use std::str::FromStr;

fn main() {
    let amt: i32 = read_input("Enter amount:");
    let rate: f64 = read_input("Enter rate:");
    let years: i32 = read_input("Enter years:");
    let future_value = amt as f64 * (1.0 + 0.01*rate).powi(years);
    println!("Future Value is {}", future_value);
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
