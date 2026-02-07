use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a:");
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let a: i64 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    input.clear();

    println!("Enter b:");
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let b: i64 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let yes = (b != 0 && a % b == 0) || (a != 0 && b % a == 0);

    if yes {
        println!("Yes");
    } else {
        println!("No");
    }
}