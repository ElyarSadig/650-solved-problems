use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut it = input.split_whitespace();

    println!("Enter a:");
    let mut a: i32 = it.next().expect("Missing a").parse().expect("Not a valid number");

    println!("Enter b:");
    let mut b: i32 = it.next().expect("Missing b").parse().expect("Not a valid number");

    println!("Enter c:");
    let mut c: i32 = it.next().expect("Missing c").parse().expect("Not a valid number");

    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    if a > c {
        std::mem::swap(&mut a, &mut c);
    }
    if b > c {
        std::mem::swap(&mut b, &mut c);
    }

    println!("Sorted is {} {} {}", a, b, c);
}