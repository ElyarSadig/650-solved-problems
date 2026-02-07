use std::io::{self, Read};

fn main() {
    println!("Enter a sentence:");
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("Failed to read input");
    let s = s.trim_end_matches(&['\n', '\r'][..]);

    let mut count: i32 = 0;
    for ch in s.chars() {
        if ch >= '0' && ch <= '9' {
            count += 1;
        }
    }

    println!("Count is {}", count);
}