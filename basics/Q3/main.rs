use std::io;

fn main() {
    const PI: f64 = 22.0 / 7.0; 
    let mut input = String::new();

    println!("Enter Radius:");
    io::stdin().read_line(&mut input).expect("could not read line");
    let radius: f64 = input.trim().parse().expect("Not a valid number");

    let volume = (4.0 / 3.0) * PI * radius * radius * radius;
    let surface_area = 4.0 * PI * radius * radius;
    
    println!("Surface Area is: {surface_area}");
    println!("Volume is: {volume}");
}