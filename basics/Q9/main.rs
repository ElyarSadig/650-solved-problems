use std::io;
use std::str::FromStr;

fn main() {
    let salary: f64 = read_input("Enter Salary:");
    let b = salary * 0.7;
    let m = salary * 0.1;
    let p = salary - b - m;
    println!("Salary: {salary} b = {b} m = {m} p = {p}");
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
