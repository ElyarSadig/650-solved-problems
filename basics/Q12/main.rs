use std::io;
use std::str::FromStr;

fn main() {
    let num: i32 = read_input("Enter number:");
    let n1 = num % 10;
    let n2 = num / 10;
    println!("Reverse is {} \t Sum is {}", n1 * 10 + n2, n1 + n2);
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
