use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter Count:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let count: i32 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter Price:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let price: i32 = input.trim().parse().expect("Not a valid number");

    let sells = price * count;

    println!("Sells is {}", sells);
}