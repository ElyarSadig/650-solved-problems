use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    println!("Enter grade for quiz one:");
    let quiz1: f64 = it
        .next()
        .expect("Missing quiz one grade")
        .parse()
        .expect("Not a valid number");

    println!("Enter grade for quiz two:");
    let quiz2: f64 = it
        .next()
        .expect("Missing quiz two grade")
        .parse()
        .expect("Not a valid number");

    println!("Enter grade for mid term:");
    let mid_term: f64 = it
        .next()
        .expect("Missing mid term grade")
        .parse()
        .expect("Not a valid number");

    println!("Enter grade for final:");
    let final_exam: f64 = it
        .next()
        .expect("Missing final grade")
        .parse()
        .expect("Not a valid number");

    let grade = (quiz1 + quiz2) * 5.0 * 0.25 + mid_term * 0.25 + final_exam * 0.5;

    if grade >= 90.0 {
        println!("Grade is A");
    } else if grade >= 80.0 {
        println!("Grade is B");
    } else if grade >= 70.0 {
        println!("Grade is C");
    } else if grade >= 60.0 {
        println!("Grade is D");
    } else {
        println!("Grade is E");
    }
}