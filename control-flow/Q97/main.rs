use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut iter = input.chars();

    loop {
        println!("Enter a char:");
        let mut ch_opt = None;

        while let Some(c) = iter.next() {
            if c == '\n' || c == '\r' || c.is_whitespace() {
                continue;
            }
            ch_opt = Some(c);
            break;
        }

        let ch = match ch_opt {
            Some(c) => c,
            None => break,
        };

        match ch {
            'w' | 'W' => println!("You love white color"),
            'r' | 'R' => println!("You love Red color"),
            'y' | 'Y' => println!("You love Yellow color"),
            'b' | 'B' => println!("You love Blue color"),
            'g' | 'G' => println!("You love Green color"),
            'e' | 'E' => break,
            _ => println!("No color chosen"),
        }
    }
}