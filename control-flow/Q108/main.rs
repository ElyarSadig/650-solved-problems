use std::io::{self, Write};

fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(v) = input.trim().parse::<f64>() {
            return v;
        }
    }
}

fn main() {
    let mut sum_miles: f64 = 0.0;
    let mut sum_gallons: f64 = 0.0;

    loop {
        let gallon = read_f64("Enter the gallons used (-1 to end): ");
        if gallon == -1.0 {
            break;
        }

        let mile = read_f64("Enter the miles driven: ");
        let rate = mile / gallon;
        println!("The miles / gallons for this tank was {}", rate);

        sum_gallons += gallon;
        sum_miles += mile;
    }

    let average = sum_miles / sum_gallons;
    println!("The overall average miles / gallons was {}", average);
}