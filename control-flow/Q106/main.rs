use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i64 = input.trim().parse().expect("Not a valid number");

    let mut f1: i64 = 0;
    let mut f2: i64 = 1;
    let mut f3: i64 = 1;

    while f3 < num {
        f3 = f1 + f2;
        f1 = f2;
        f2 = f3;
    }

    if f3 == num {
        println!("YES");
    } else {
        println!("NO");
    }
}