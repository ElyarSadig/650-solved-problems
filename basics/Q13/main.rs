use std::io;
use std::str::FromStr;

fn main() {
    let x: i32 = read_input("Enter x:");
    let m = (x << 5) - x;
    let n = -((x << 4) + x);
    let y = m + n + 5;
    println!("y = {}", y);
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
