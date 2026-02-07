use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    loop {
        println!("Enter m:");
        let m: f64 = match it.next() {
            Some(v) => v.parse().expect("Not a valid number"),
            None => return,
        };

        println!("Enter tedad:");
        let tedad: i32 = match it.next() {
            Some(v) => v.parse().expect("Not a valid number"),
            None => return,
        };

        println!("Enter s:");
        let s: f64 = match it.next() {
            Some(v) => v.parse().expect("Not a valid number"),
            None => return,
        };

        let k = m + (m * ((tedad as f64) + 1.0) * s) / 2400.0;
        let p = k / (tedad as f64);
        println!("k = {}\t{}", k, p);

        println!("Do you want to continue(y/n):");
        let ansi = match it.next() {
            Some(v) => v,
            None => return,
        };

        if ansi.as_bytes().first() == Some(&b'n') {
            break;
        }
    }
}