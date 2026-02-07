use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter x:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: f64 = input.trim().parse().expect("Not a valid number");

    let mut sign: f64 = 1.0;
    let mut pow: f64 = 1.0;
    let mut sum: f64 = 0.0;
    let mut sum1: f64 = 0.0;

    for i in 1..=10 {
        pow *= x;
        sum1 += (i as f64) * pow;
        sum += sign * (1.0 / sum1);
        sign = -sign;
    }

    println!("S = {}", sum);
}