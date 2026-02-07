use std::io;

fn main() {
    let _ = io::stdin();

    let s1: i32 = 13 * 16;
    println!("s1 = {}", s1);

    let s2: i32 = 2 * 3;
    println!("s2 = {}", s2);

    let b: i32 = s1 / s2;
    let a: i32 = s1 % s2;

    println!("b = {}", b);
    println!("a = {}", a);
}