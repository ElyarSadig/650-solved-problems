use std::io;

fn main() {
    let _ = io::stdin();
    for i in (5..=8).rev() {
        for j in (5..=8).rev() {
            for k in (5..=8).rev() {
                for m in (5..=8).rev() {
                    if i == j || i == k || i == m || j == k || j == m || k == m {
                        continue;
                    }
                    println!("{}", i * 1000 + j * 100 + k * 10 + m);
                }
            }
        }
    }
}