use std::io;

fn main() {
    let _ = io::stdin();
    for i in 0u32..256u32 {
        let ch = std::char::from_u32(i).unwrap_or('\u{FFFD}');
        println!("{} = {}", i, ch);
    }
}