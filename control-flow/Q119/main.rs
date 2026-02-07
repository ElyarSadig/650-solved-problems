use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Not a valid number");

    let mut sum: f64 = 0.0;
    let mut sign: i32 = 1;
    let mut fact: f64 = 1.0;

    for i in 1..=n {
        fact *= i as f64;
        if sign == 1 {
            print!(" +{}/{}", i, fact);
        } else {
            print!(" -{}/{}", i, fact);
        }
        sum += (sign * i) as f64 / fact;
        sign = -sign;
    }

    println!(" ={}", sum);
}