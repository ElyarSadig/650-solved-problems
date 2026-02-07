use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter a number:");
    let num: i32 = it
        .next()
        .expect("Missing number")
        .parse()
        .expect("Not a valid number");

    println!("Enter nth bit to check (0-31):");
    let n: u32 = it
        .next()
        .expect("Missing bit index")
        .parse()
        .expect("Not a valid number");

    let new_num = (1i32.checked_shl(n).expect("Shift overflow")) | num;

    println!("Number before setting {} bit: {} (in decimal)", n, num);
    println!("Number after setting {} bit: {} (in decimal)", n, new_num);
}