use std::io;
use std::str::FromStr;

fn main() {
    let x: i32;
    x = read_input("Enter x:");
    println!("{} ^ 2 = {}\n{}^ 3 = {}", x, x * x, x, x * x * x);
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
