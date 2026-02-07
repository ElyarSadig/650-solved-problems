use std::io;
use std::time::{Duration, SystemTime};

fn main() {
    let mut input = String::new();

    println!("Enter n:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Not a valid number");

    let now = SystemTime::now();
    let new_time = if n >= 0 {
        now.checked_add(Duration::from_secs(n as u64)).expect("Time overflow")
    } else {
        now.checked_sub(Duration::from_secs((-n) as u64)).expect("Time overflow")
    };

    println!("Current Time: {:?}", now);
    println!("New Time: {:?}", new_time);
}