use std::io;
use std::str::FromStr;
use num::complex::Complex;

fn main() {
    let real: f64 = read_input("Enter real part:");
    let img: f64 = read_input("Enter image part:");

    let complex = Complex::new(real, img);
    println!("complex: {}", complex)
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
