use std::io;

fn main() {
    let _ = io::stdin();

    for i in 1..=8 {
        for _ in (0..=8 - i).rev() {
            print!("{} ", i);
        }
        println!();
    }
}