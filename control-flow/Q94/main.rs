use std::io::{self, Read};

fn main() {
    let mut _input = String::new();
    let _ = io::stdin().read_to_string(&mut _input);

    for i in (1..=7).rev() {
        for j in (1..=i).rev() {
            print!("{} ", j);
        }
        println!();
    }
}