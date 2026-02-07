use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter year:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let year: f64 = input.trim().parse().expect("Not a valid number");

    let day: f64 = 365.25 * year;
    println!("Day is {}", day);

    let month: i32 = (day / 30.0) as i32;
    println!("Month is {}", month);

    let seconds: f64 = day * 24.0 * 3600.0;
    println!("Seconds is {}", seconds);
}