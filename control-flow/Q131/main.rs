use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut n: i64 = input.trim().parse().expect("Not a valid number");

    let mut count_odd: i64 = 0;
    let mut count_even: i64 = 0;
    let mut sum_odd: i64 = 0;
    let mut sum_even: i64 = 0;

    println!("Number\tDigit");
    println!("======\t=====");

    while n > 0 {
        let digit = n % 10;
        println!("{}\t{}", n, digit);

        if digit % 2 == 0 {
            count_even += 1;
            sum_even += digit;
        } else {
            count_odd += 1;
            sum_odd += digit;
        }

        n /= 10;
    }

    let avg_even = (sum_even as f64) / (count_even as f64);
    let avg_odd = (sum_odd as f64) / (count_odd as f64);

    println!("Average Even is: {}", avg_even);
    println!("Average Odd is: {}", avg_odd);
}