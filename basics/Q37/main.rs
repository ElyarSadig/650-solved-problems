use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter r:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let r: f64 = input.trim().parse().expect("Not a valid number");

    let n = 2.0 * r;
    let p = std::f64::consts::PI * n;
    let a = std::f64::consts::PI * r * r;

    print!("n = {}\np = {}\na = {}", n, p, a);
}