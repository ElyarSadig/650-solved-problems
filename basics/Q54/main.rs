use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter p:");
    let p: f64 = it.next().expect("Missing p").parse().expect("Not a valid number");

    println!("Enter h:");
    let h: f64 = it.next().expect("Missing h").parse().expect("Not a valid number");

    let s = p * h / 2.0;
    println!("S is {}", s);
}