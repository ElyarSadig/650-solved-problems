use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Not a valid number");

    let y = a * b.sin();

    println!("y is {}", y);
}