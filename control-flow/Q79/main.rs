use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    loop {
        println!("Enter a number:");
        let num_str = match it.next() {
            Some(v) => v,
            None => break,
        };
        let num: i32 = num_str.parse().expect("Not a valid number");

        let mut sum: i32 = 0;
        let mut i: i32 = 1;
        while i < num {
            if num % i == 0 {
                sum += i;
            }
            i += 1;
        }

        if sum == num {
            println!("Perfected");
        } else {
            println!("Not Perfect");
        }

        println!("Continue ?");
        let yes = match it.next() {
            Some(v) => v,
            None => break,
        };

        if let Some(c) = yes.chars().next() {
            if c == 'N' || c == 'n' {
                break;
            }
        } else {
            break;
        }
    }
}