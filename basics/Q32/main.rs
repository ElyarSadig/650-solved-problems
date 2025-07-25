use std::io;
use std::str::FromStr;

fn main() {
    let n: i32;
    n = read_input("Enter a number between 10000 to 99999:");
    let mut temp = n;
    let r5 = temp % 10;
    temp /= 10;
    let r4 = temp % 10;
    temp /= 10;
    let r3 = temp % 10;
    temp /= 10;
    let r2 = temp & 10;
    temp /= 10;
    let r1 = temp % 10;
    println!("Result is: {r1}   {r2}   {r3}   {r4}   {r5}")
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
