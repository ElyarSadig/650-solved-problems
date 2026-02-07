use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter pages:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let pages: i64 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter size of memory:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let gb: i64 = input.trim().parse().expect("Not a valid number");

    let book_bytes: i64 = 80 * 30 * pages;
    let total_bytes: i64 = 1024 * 1024 * 1024 * gb;

    let number: f64 = total_bytes as f64 / book_bytes as f64;

    println!("Number of book is {}", number);
}