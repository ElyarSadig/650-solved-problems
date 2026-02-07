use std::io;

fn read_i32(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Not a valid number")
}

fn main() {
    let mut num1: i32 = read_i32("Enter num1:");
    let mut num2: i32 = read_i32("Enter num2:");

    num1 ^= num2;
    num2 ^= num1;
    num1 ^= num2;

    println!("Num1 after swapping = {}", num1);
    println!("Num2 after swapping = {}", num2);
}