use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter a:");
    let a: i32 = match it.next() {
        Some(v) => v.parse().expect("Not a valid number"),
        None => return,
    };

    println!("Enter op:");
    let ch: String = match it.next() {
        Some(v) => v.to_string(),
        None => return,
    };

    println!("Enter b:");
    let b: i32 = match it.next() {
        Some(v) => v.parse().expect("Not a valid number"),
        None => return,
    };

    let result: i32 = match ch.as_str() {
        "+" | "a" | "A" => a + b,
        "-" | "S" | "s" => a - b,
        "*" | "X" | "x" => a * b,
        "/" | "D" | "d" => a / b,
        "%" | "M" | "m" => a % b,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    print!("{} {} {} = {}", a, ch, b, result);
}