use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let n: i32 = input
        .split_whitespace()
        .next()
        .expect("No input provided")
        .parse()
        .expect("Not a valid number");

    println!("Enter N:");

    for i in 1..=n {
        println!("{}\t{}\t{}\t{}", i, i * 10, i * 100, i * 1000);
    }
}