use std::io;

fn main() {
    let _ = io::stdin();
    for i in 1..=8 {
        for _j in 1..=8 {
            print!("{} ", i);
        }
        println!();
    }
}