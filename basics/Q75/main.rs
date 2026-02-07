use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter degree:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let degree: f64 = input.trim().parse().expect("Not a valid number");

    let radian = degree * (std::f64::consts::PI / 180.0);

    println!("Radian is: {}", radian);
}