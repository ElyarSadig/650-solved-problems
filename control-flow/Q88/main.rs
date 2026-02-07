use std::io::{self, Write};

fn read_int(prompt: &str) -> i32 {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(v) = input.trim().parse::<i32>() {
            return v;
        }
    }
}

fn main() {
    loop {
        let x = read_int("Enter x: ");
        let y = read_int("Enter y: ");

        if x == 0 && y == 0 {
            break;
        }

        let mut sum: i32 = 0;
        let temp = y.abs();
        for _ in 0..temp {
            sum = sum.wrapping_add(x);
        }
        if y < 0 {
            sum = sum.wrapping_neg();
        }

        println!("{} * {} = {}", x, y, sum);
    }
}