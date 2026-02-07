use std::io;

fn min3(a: i32, b: i32, c: i32) -> i32 {
    let ab = if a < b { a } else { b };
    if ab < c { ab } else { c }
}

fn max3(a: i32, b: i32, c: i32) -> i32 {
    let ab = if a > b { a } else { b };
    if ab > c { ab } else { c }
}

fn read_i32(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Not a valid number")
}

fn main() {
    let x = read_i32("Enter first number:");
    let y = read_i32("Enter second number:");
    let z = read_i32("Enter third number:");

    let a1 = min3(x, y, z);
    let a3 = max3(x, y, z);
    let a2 = (x + y + z) - a1 - a3;

    println!("Numbers in sorted order: {} {} {}", a1, a2, a3);
}