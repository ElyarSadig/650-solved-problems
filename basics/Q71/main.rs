use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Not a valid number");

    let flipped_number = !num;

    println!("Original Number = {}", num);
    println!("Number after bits are flipped = {}", flipped_number);
}