use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter n:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Not a valid number");

    let mut sum: i32 = 0;

    for i in 1..=n {
        if i < n {
            print!("{} + ", i);
        } else {
            print!("{} = ", i);
        }
        sum += i;
    }

    print!("{}", sum);
}