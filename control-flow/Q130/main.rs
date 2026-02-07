use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter a number:");
    let mut n: i64 = it.next().unwrap_or("").parse().expect("Not a valid number");

    let mut multiply: i64 = 1;

    println!("Number\t\tDigit\t\tMultiply");
    println!("======\t\t======\t\t======");

    while n > 0 {
        let digit = n % 10;
        if digit % 2 != 0 {
            multiply *= digit;
        }
        println!("{}\t\t{}\t\t{}", n, digit, multiply);
        n /= 10;
    }

    println!("Multiply is {}", multiply);
}