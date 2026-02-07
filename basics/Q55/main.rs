use std::io;

fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<f64>().expect("Not a valid number")
}

fn main() {
    let h = read_f64("Enter hour:");
    let r = read_f64("Enter rate:");

    let tp = h * r;
    let t = tp / 10.0;
    let pp = tp - t;

    println!("Total Payment: {}", tp);
    println!("Tax:  {}", t);
    println!("Payment:  {}", pp);
}