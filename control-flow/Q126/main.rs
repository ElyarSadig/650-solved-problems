use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("Enter month: ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        if stdin.read_line(&mut input).expect("Failed to read line") == 0 {
            return;
        }

        let month: i32 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => return,
        };

        match month {
            1 => println!("Farvardin"),
            2 => println!("Ordibehesht"),
            3 => println!("Khordad"),
            4 => println!("Tir"),
            5 => println!("Mordad"),
            6 => println!("Shahrivar"),
            7 => println!("Mehr"),
            8 => println!("Aban"),
            9 => println!("Azar"),
            10 => println!("Dey"),
            11 => println!("Bahman"),
            12 => println!("Esfand"),
            _ => return,
        }
    }
}