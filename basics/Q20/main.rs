use std::io;
use std::str::FromStr;

fn main() {
    let kg: f64 = read_input("Enter Kg:");
    let g = kg * 1000.0;
    println!("Weight(g) = {}", g)
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
