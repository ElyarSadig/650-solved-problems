use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter m:");
    let m: i64 = it.next().expect("Missing m").parse().expect("Not a valid number");

    println!("Enter n:");
    let n: i64 = it.next().expect("Missing n").parse().expect("Not a valid number");

    let mut temp = m;

    if n == 0 {
        temp = 1;
    } else {
        for _ in 0..(n - 1) {
            let mut sum = 0i64;
            for _ in 0..temp {
                sum = sum + m;
            }
            temp = sum;
        }
    }

    print!("{} ^ {} = {}", m, n, temp);
}