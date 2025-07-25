use std::io;
use std::str::FromStr;

fn main() {
    let liter: f64;
    let mile: f64;
    liter = read_input("Enter liter:");
    mile = read_input("Enter mile:");
    let result = (mile/liter) * 0.265179;
    println!("miles / gallons is: {result}")
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
