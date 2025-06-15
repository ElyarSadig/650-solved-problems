use std::io;
use std::str::FromStr;

fn main() {
    let salary: f64 = read_input("Enter Salary:");
    let reward = salary * 0.15;
    println!("Reward is: {}", reward)
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
