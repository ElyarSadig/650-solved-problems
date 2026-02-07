use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your age:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let year: f64 = input.trim().parse().expect("Not a valid number");

    let minute = year * 365.25 * 24.0 * 60.0;

    println!("Minute is {}", minute);
}