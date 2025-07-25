use std::io;
use std::str::FromStr;

fn main() {
    let a: i32;
    let b: String;
    a = read_input("Enter integer for a:");
    b = read_input("Enter a name for b:");
    println!("Pointer a is {:p}", &a);
    println!("Pointer b is {:p}", &b);
}

fn read_input<T: FromStr>(prompt: &str) -> T {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<T>() {
        Ok(value) => return value,
        Err(_e) => panic!("Invalid input: please try again."),
    }
}
