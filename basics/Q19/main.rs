use std::io;
use std::str::FromStr;

fn main() {
    let r1: f64 = read_input("Enter R1:");
    let r2: f64 = read_input("Enter R2:");
    let r3: f64 = read_input("Enter R3:");
    let r = 1.0 / r1 + 1.0 / r2 + 1.0 / r3;
    println!("R = {}", 1.0 / r)
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
