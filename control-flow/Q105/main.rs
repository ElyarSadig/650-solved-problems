use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut it = input.split_whitespace();
    let cost_str = it.next().unwrap_or("");
    let mut cost: f64 = cost_str.parse().expect("Not a valid number");

    for i in 0..10 {
        cost -= cost * 20.0 / 100.0;
        println!("cost for year {}: {}", i + 1, cost);
    }
}