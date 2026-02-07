use std::io;

fn main() {
    let _ = io::stdin();

    let nums = [1, 2, 4, 9];
    let mut count = 0;

    for &i in &nums {
        for &j in &nums {
            for &c in &nums {
                for &k in &nums {
                    if i == j || i == c || i == k || j == c || j == k || c == k {
                        continue;
                    } else {
                        println!("{}", i * 1000 + j * 100 + c * 10 + k);
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Count: {}", count);
}