use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter an odd number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Not a valid number");

    let y = n / 3;
    let mut k = n + 3;

    for _ in 1..(y + 2) {
        print!(" ");
    }
    for _ in 1..(y + 1) {
        print!("*");
    }
    println!();

    for i in 1..(y + 1) {
        for _ in 1..(y - i + 1) {
            print!(" ");
        }
        print!("*");
        for _ in (n + 2)..(y + k + 1) {
            print!(" ");
        }
        if i == n {
            print!(" ");
        } else {
            print!("*");
        }
        println!();
        k += 2;
    }

    for _ in 1..(n / 2 + 1) {
        print!("*");
        for _ in 1..(n + 1) {
            print!(" ");
        }
        println!("*");
    }

    let mut m = 1;
    for i in 1..(y + 1) {
        for _ in (y + 2)..(y + m + 1) {
            print!(" ");
        }
        print!(" ");
        if i == y {
            print!(" ");
            for _ in 1..(y + 1) {
                print!("*");
            }
        } else {
            print!("*");
        }
        for _ in 1..(2 * y - 2 * i + y + 1) {
            print!(" ");
        }
        if i == y {
            print!("");
        } else {
            print!("*");
        }
        println!();
        m += 1;
    }
}