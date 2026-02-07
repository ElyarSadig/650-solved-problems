use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter x:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i64 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter y:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y: i64 = input.trim().parse().expect("Not a valid number");

    let z = x * x * x + 2 * x * x + 3 * y - 5;

    println!("z is {}", z);
}