use std::io;
use std::str::FromStr;

fn main() {
    let a: i32 = read_input("Enter A4 price:");
    let pen: i32 = read_input("Enter pen price:");
    let t: f64 = read_input("Enter t:");
    let cost = 50.0 * pen as f64 * t / 100.0 + 150.0 * a as f64 * t / 100.0;
    println!("Extra cost is {}", cost)
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
