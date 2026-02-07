use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter n:");
    let n: i64 = it.next().expect("Missing n").parse().expect("Not a valid number");

    let mut sum: i64 = 0;
    for _ in 0..n {
        println!("Enter a number:");
        let num: i64 = it.next().expect("Missing number").parse().expect("Not a valid number");
        sum += num;
    }

    println!("Sum is {}", sum);
    let avg = (sum as f64) / (n as f64);
    println!("Average is {}", avg);
}