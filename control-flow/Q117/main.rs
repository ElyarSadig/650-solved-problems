use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let n: i32 = input.split_whitespace().next().unwrap().parse().expect("Not a valid number");

    println!("Enter n:");

    let mut sign: i32 = -1;
    let mut sum: i32 = 0;

    for i in 1..=n {
        if sign == 1 && i != n {
            print!("{} + ", i);
        } else if i != n {
            print!("{} - ", i);
        } else {
            print!("{} = ", i);
        }
        sum += sign * i;
        sign *= -1;
    }
    print!("{}", sum);
}