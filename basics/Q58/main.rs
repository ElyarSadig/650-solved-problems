use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter a:");
    let a: f64 = it.next().expect("Missing a").parse().expect("Not a valid number");

    println!("Enter b:");
    let b: f64 = it.next().expect("Missing b").parse().expect("Not a valid number");

    println!("Enter c:");
    let c: f64 = it.next().expect("Missing c").parse().expect("Not a valid number");

    let x = (b - c) / (3.0 - a);
    let y = 3.0 * x + c;

    print!("({:.6}, {:.6})", x, y);
}