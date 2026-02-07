use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter Radian:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let radian: f64 = input.trim().parse().expect("Not a valid number");

    let degree = radian * (180.0 / std::f64::consts::PI);

    println!("Degree is {}", degree);
}