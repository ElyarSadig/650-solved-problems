use std::io;

fn main() {
    let mut _input = String::new();
    let _ = io::stdin().read_line(&mut _input);

    for i in 1..=10 {
        for j in 1..=5 {
            for k in 1..=2 {
                if i * 1000 + j * 2000 + k * 5000 == 10000 {
                    println!("1000 * {} + 2000 * {} + 5000 * {} = {}", i, j, k, 10000);
                }
            }
        }
    }
}