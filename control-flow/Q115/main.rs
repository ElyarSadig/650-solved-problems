use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    loop {
        println!("Enter account number(-1 to end):");
        let account_str = match it.next() {
            Some(v) => v,
            None => break,
        };
        let account: i32 = account_str.parse().expect("Not a valid number");
        if account == -1 {
            break;
        }

        println!("Enter begining balance:");
        let begining_balance: f64 = it
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not a valid number");

        println!("Enter total charges:");
        let total_charge: f64 = it
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not a valid number");

        println!("Enter total credits:");
        let total_credits: f64 = it
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not a valid number");

        println!("Enter credit limit:");
        let credit_limit: f64 = it
            .next()
            .expect("Missing value")
            .parse()
            .expect("Not a valid number");

        let balance = begining_balance + total_credits - total_charge;

        if balance > credit_limit {
            println!("Account: {}", account);
            println!("Credit Limit: {}", credit_limit);
            println!("lance: {}", balance);
            println!("Credit Limit Exceeded");
        }
    }
}