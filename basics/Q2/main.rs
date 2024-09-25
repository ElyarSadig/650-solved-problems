use std::io;

fn main() {
    let mut input = String::new();

    const PI: f64 = 22.0 / 7.0;

    println!("Height of cylinder:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: f64 = input.trim().parse().expect("Not a valid number");

    input.clear();

    println!("Radius of cylinder:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let radius: f64 = input.trim().parse().expect("Not a valid number");

    let volume = PI * radius * radius * height;
    let surface_area = ((2.0 * PI * radius) * height) + (2.0 * (PI * radius * radius));
    
    println!("Volume is: {volume}");
    println!("Surface Area is: {surface_area}");
}
