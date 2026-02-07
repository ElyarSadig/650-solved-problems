use std::io;

fn read_i32(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Not a valid number")
}

fn main() {
    const M_PER_MILE: f64 = 1609.35;
    const M_PER_FOOT: f64 = 0.30480;

    let miles = read_i32("Enter the number of miles:");
    let feet = read_i32("Enter the number of feet:");

    let total_meters = (miles as f64) * M_PER_MILE + (feet as f64) * M_PER_FOOT;
    let total_kilometers = total_meters / 1000.0;

    let kilometers = total_kilometers as i32;
    let meters = (total_kilometers - (kilometers as f64)) * 1000.0;

    println!(
        "The distance is {} kilometers, {} meters.",
        kilometers, meters
    );
}