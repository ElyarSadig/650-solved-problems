use std::io::{self, Write};

fn main() {
    let mut out = String::new();

    for i in 1..=8 {
        for _ in 0..i {
            out.push_str(&format!("{} ", i));
        }
        out.push('\n');
    }

    let mut stdout = io::stdout();
    stdout.write_all(out.as_bytes()).expect("Failed to write output");
}