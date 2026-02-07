use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter n: ");
    let n: i32 = it.next().expect("Missing n").parse().expect("Not a valid number");

    let mut base6: i32 = 0;
    let mut total_w: i32 = 0;
    let mut total_m: i32 = 0;
    let mut sum_m: f64 = 0.0;
    let mut sum_w: f64 = 0.0;

    for _ in 0..n {
        println!("Enter an id: ");
        let _id: i32 = it.next().expect("Missing id").parse().expect("Not a valid number");

        println!("Enter a date: ");
        let _date: i32 = it.next().expect("Missing date").parse().expect("Not a valid number");

        println!("Enter a m_w(0|1): ");
        let m_w: i32 = it.next().expect("Missing m_w").parse().expect("Not a valid number");

        println!("Enter base: ");
        let base1: i32 = it.next().expect("Missing base").parse().expect("Not a valid number");

        println!("Enter payment: ");
        let pay: f64 = it.next().expect("Missing payment").parse().expect("Not a valid number");

        if base1 == 6 {
            base6 += 1;
        }

        if m_w == 0 {
            sum_w += pay;
            total_w += 1;
        } else {
            sum_m += pay;
            total_m += 1;
        }
    }

    println!("********* Result ********* \n Number of base six is {}", base6);
    println!("Number of women is {}", total_w);

    if total_w > 0 {
        println!("Average(woman) pay is {}", sum_w / (total_w as f64));
    }
    if total_m > 0 {
        println!("Average(man) pay is {}", sum_m / (total_m as f64));
    }
}