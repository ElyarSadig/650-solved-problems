use std::io;

fn main() {
    let _ = io::stdin();
    println!("Result is:");
    for i in 1000..=1100 {
        if i % 9 == 0 {
            print!("{} ", i);
        }
    }
}