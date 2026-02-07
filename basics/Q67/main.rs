use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Enter nth bit to check (0-31):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Not a valid number");

    let new_num = num & !(1i32 << n);

    println!("Number before clearing {} bit: {}", n, num);
    println!("Number after clearing {} bit: {}", n, new_num);
}