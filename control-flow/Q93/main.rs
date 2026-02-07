use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().expect("Failed to flush");

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let s = input.trim_end_matches(&['\n', '\r'][..]);

    let vowels: [char; 10] = ['a', 'e', 'o', 'u', 'i', 'A', 'E', 'O', 'U', 'I'];
    let mut count: i32 = 0;

    for ch in s.chars() {
        for v in vowels.iter() {
            if *v == ch {
                count += 1;
            }
        }
    }

    println!("Count is {}", count);
}

use std::io::Write;