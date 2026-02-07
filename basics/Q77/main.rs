use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter a:");
    let a_str = it.next().expect("Missing a");
    let _a: i32 = a_str.parse().expect("Not a valid number");

    println!("Enter b:");
    let b_str = it.next().expect("Missing b");
    let _b: f64 = b_str.parse().expect("Not a valid number");

    println!("Enter c:");
    let _c: String = it.next().expect("Missing c").to_string();

    println!("Type a is i32");
    println!("Type b is f64");
    println!("Type c is String");
}