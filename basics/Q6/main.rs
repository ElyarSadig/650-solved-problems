use std::io;
use std::str::FromStr;

fn main() {
    let input_string: String = read_input("Enter a string: ");
    let repeat: isize = read_input("Enter repeat: ");

    for _ in 0..repeat {
        print!("{input_string}");
    }
    println!()
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
