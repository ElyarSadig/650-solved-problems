use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter x:");
    let x_str = it.next().expect("Missing x");
    let mut x: i64 = x_str.parse().expect("Not a valid number");

    println!("Enter y:");
    let y_str = it.next().expect("Missing y");
    let mut y: i64 = y_str.parse().expect("Not a valid number");

    x = x + y;
    y = x - y;
    x = x - y;

    println!("X = {} Y = {}", x, y);
}