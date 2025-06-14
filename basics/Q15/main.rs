use std::io;
use std::str::FromStr;

fn main() {
    let n: i32 = read_input("Enter n:");
    let n1 = n;
    let n2 = n1 * 10 + n;
    let n3 = n2 * 10 + n;
    println!("{n1} + {n2} + {n3} = {}", n1 + n2 + n3)
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
