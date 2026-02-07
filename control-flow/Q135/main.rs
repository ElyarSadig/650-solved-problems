use std::io;

fn main() {
    let _ = io::stdin();
    for i in (2..=14).step_by(4) {
        let mut temp = i;
        for _ in 1..=6 {
            print!("{}\t", temp);
            temp += 2;
        }
        println!();
    }
}