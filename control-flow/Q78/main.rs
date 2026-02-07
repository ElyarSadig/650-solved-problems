use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter n:");
    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    if n < 2 {
        println!("Please enter a number greater than 1");
        return;
    }

    let mut id1: i32 = -1;
    let mut id2: i32 = -1;
    let mut max1: f64 = -1.0;
    let mut max2: f64 = -1.0;

    for _ in 0..n {
        println!("Enter id:");
        let id: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };

        println!("Enter average:");
        let avg: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };

        if avg > max1 {
            max2 = max1;
            max1 = avg;
            id2 = id1;
            id1 = id;
        } else if avg > max2 {
            id2 = id;
            max2 = avg;
        }
    }

    println!("Max2 = {}\t Id2 = {}", max2, id2);
}