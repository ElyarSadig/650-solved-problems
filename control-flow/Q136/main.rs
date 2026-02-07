use std::io;

fn main() {
    let _ = io::stdin();
    for i in (100..=500).step_by(50) {
        for j in (i..=i + 500).step_by(100) {
            print!("{}  ", j);
        }
        println!();
    }
}