use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Not a valid number");

    let mut f1: i32 = 1;
    let mut f2: i32 = 1;

    if n == 1 {
        println!("{}", f1);
    } else if n == 2 {
        println!("{}", f1);
        println!("{}", f2);
    } else {
        println!("{}", f1);
        println!("{}", f2);
        let mut f3: i32;
        for _ in 3..=n {
            f3 = f1 + f2;
            println!("{}", f3);
            f1 = f2;
            f2 = f3;
        }
    }
}