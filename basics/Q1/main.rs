use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter base:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let base: f64 = input.trim().parse().expect("Not a valid number");

    input.clear();
    
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: f64 = input.trim().parse().expect("Not a valid number");
    
    let area = base * height;
    
    println!("Area is: {}", area);
}
