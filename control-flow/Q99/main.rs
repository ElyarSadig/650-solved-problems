use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter n:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Not a valid number");

    let mut fact: i64 = 1;
    let mut sum: f64 = 0.0;

    let mut i: i32 = 1;
    while i < n {
        fact *= i as i64;
        sum += 1.0 / fact as f64;
        i += 1;
    }

    println!("Sum is: {}", sum);
}