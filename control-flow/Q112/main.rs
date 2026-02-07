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

    println!("Enter b:");
    let b: i32 = match it.next() {
        Some(v) => v.parse().expect("Not a valid number"),
        None => return,
    };

    println!("Enter c:");
    let c: i32 = match it.next() {
        Some(v) => v.parse().expect("Not a valid number"),
        None => return,
    };

    if a > b + c || c > b + a || b > c + a {
        println!("NO");
    } else {
        println!("YES");
    }
}