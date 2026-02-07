use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let num: i32 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    input.clear();

    println!("Enter nth bit to check (0-31):");
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let n: u32 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let bit_status = ((num >> n) & 1) as i32;

    println!("The {} bit is set to {}", n, bit_status);
}