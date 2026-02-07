use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i64 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: i64 = input.trim().parse().expect("Not a valid number");

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);

    if b == 0 {
        eprintln!("a / b = division by zero");
    } else {
        println!("a / b = {}", a / b);
    }

    println!("a^2 + b^2 = {}", a * a + b * b);
    println!("a^3 + b^3 = {}", a * a * a + b * b * b);
}