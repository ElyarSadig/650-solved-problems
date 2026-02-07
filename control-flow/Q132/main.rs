use std::io;

fn main() {
    let _ = io::stdin();
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                print!("{}\t", i * 100 + j * 10 + k);
            }
            println!();
        }
    }
}