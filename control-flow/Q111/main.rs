use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap_or("").parse().expect("Not a valid number");

    println!("Enter n:");
    let mut sign: f64 = -1.0;
    let mut p: f64 = 4.0;
    let mut k: f64 = 3.0;

    println!("i    \tPI");
    println!("===   \t===================");
    for i in 1..=n {
        p += sign * 4.0 / k;
        k += 2.0;
        sign *= -1.0;
        println!("{}\t {}", i, p);
    }
}