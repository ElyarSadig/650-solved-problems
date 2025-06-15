use std::io;
use std::str::FromStr;

fn main() {
    let consumption: f64 = read_input("Enter Consumption:");
    let capacity: f64 = read_input("Enter Capacity:");
    let dist = capacity / consumption * 100.0;
    println!("Distance is {}", dist);
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
