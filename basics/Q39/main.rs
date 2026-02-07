use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter m:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let m: i64 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter n:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Not a valid number");

    let a = m * m - n * n;
    let b = 2 * m * n;
    let c = m * m + n * n;

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}