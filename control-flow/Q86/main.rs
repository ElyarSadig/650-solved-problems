use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter n:");
    let n: i32 = match it.next() {
        Some(v) => v.parse().expect("Not a valid number"),
        None => return,
    };

    for _ in 1..=n {
        println!("Enter id:");
        let id: i32 = it.next().expect("Missing input").parse().expect("Not a valid number");

        println!("Enter h:");
        let h: i32 = it.next().expect("Missing input").parse().expect("Not a valid number");

        println!("Enter hp:");
        let hp: i32 = it.next().expect("Missing input").parse().expect("Not a valid number");

        let mut ov: f64 = 0.0;
        if h > 40 {
            ov = 0.5 * ((h - 40) * hp) as f64;
        }
        let p: f64 = ov + (hp * h) as f64;

        println!("id = {}  ov = {}  p = {}", id, ov, p);
    }
}