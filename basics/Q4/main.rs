use std::{f64::consts::PI, io};

fn main() {
    let mut input = String::new();
    let n_sides: u32;
    let s_length: f64;

    println!("Input number of sides: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    n_sides = input.trim().parse().expect("Not a valid number for n_sides");

    input.clear();

    println!("Input the length of a side: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    s_length = input.trim().parse().expect("Not a valid number for s_length");

    let p_area = (n_sides as f64 * (s_length * s_length)) / (4.0 * (PI / n_sides as f64).tan());

    println!("The area of the polygon is: {}", p_area);
}
