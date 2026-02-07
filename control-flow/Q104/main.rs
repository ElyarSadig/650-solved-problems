use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    loop {
        print!("Enter a char: ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            return;
        }

        let ch = match input.chars().find(|c| *c != '\n' && *c != '\r') {
            Some(c) => c,
            None => return,
        };

        match ch {
            'b' | 'B' => println!("You selected Lady"),
            'd' | 'D' => println!("You selected Miss"),
            'p' | 'P' => println!("You selected Professor"),
            'a' | 'A' => println!("You selected Mr"),
            'j' | 'J' => println!("You selected Excellency"),
            'm' | 'M' => println!("You selected Wife"),
            _ => return,
        }
    }
}