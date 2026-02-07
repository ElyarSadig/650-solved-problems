use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter x:");
    let x: i32 = it.next().expect("Missing x").parse().expect("Not a valid number");

    println!("Enter y:");
    let y: i32 = it.next().expect("Missing y").parse().expect("Not a valid number");

    let mut i: i32 = 0;

    let (mut temp, r) = if x > y { (x, y) } else { (y, x) };

    while temp >= r {
        temp -= r;
        i += 1;
    }

    println!("{} / {} = {}", x, y, i);
}