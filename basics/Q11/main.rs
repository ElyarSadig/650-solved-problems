use std::io;
use std::str::FromStr;

fn main() {
    let tedad: i64 = read_input("Enter tedad:");
    let payment = 75000;
    let extera = (payment * 12 * tedad) as f64 * 13.5 / 100.0;
    println!("Extera is: {:10.6}", extera)
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
