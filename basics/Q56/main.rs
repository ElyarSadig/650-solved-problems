use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter x1:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x1: f64 = input.trim().parse().expect("Not a valid number");

    input.clear();
    println!("Enter x2:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x2: f64 = input.trim().parse().expect("Not a valid number");

    input.clear();
    println!("Enter y1:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y1: f64 = input.trim().parse().expect("Not a valid number");

    input.clear();
    println!("Enter y2:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y2: f64 = input.trim().parse().expect("Not a valid number");

    let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    println!("Distance is: {}", dist);
}