use std::io;
use std::str::FromStr;

fn main() {
    let m = 3e-23;
    let l = 950.0;
    let w: f64 = read_input("Input w: ");
    let tedad = (w * l) / m;
    println!("Tedad: {tedad}")
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
