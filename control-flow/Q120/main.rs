use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter x:");
    let x: f64 = it.next().expect("Missing x").parse().expect("Not a valid number");

    println!("Enter n:");
    let n: i32 = it.next().expect("Missing n").parse().expect("Not a valid number");

    let mut sign: i32 = 1;
    let mut sum: f64 = 0.0;
    let mut pow: f64 = 1.0;
    let mut fact: f64 = 1.0;

    for i in 1..=n {
        fact *= i as f64;
        pow *= x;

        if sign == 1 {
            print!(" +{}{}", pow, format!("/{}", fact));
        } else {
            print!(" -{}{}", pow, format!("/{}", fact));
        }

        sign = -sign;
        sum += pow / fact * (sign as f64);
    }

    print!(" = {}", sum);
}