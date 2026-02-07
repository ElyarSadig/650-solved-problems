use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter n:");
    let n: i32 = it.next().expect("Missing n").parse().expect("Not a valid number");

    for _ in 0..=n {
        println!("Enter num:");
        let num: i32 = it.next().expect("Missing num").parse().expect("Not a valid number");
        if num % 9 == 0 {
            println!("{}", num);
        }
    }
}