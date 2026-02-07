use std::io;

fn main() {
    let _ = io::stdin();
    for _ in 0..5 {
        for _ in 0..6 {
            print!("$");
        }
        println!();
    }
}