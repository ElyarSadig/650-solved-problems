use std::io;

fn read_int(prompt: &str) -> i64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Not a valid number")
}

fn main() {
    let a = read_int("Enter a:");
    let b = read_int("Enter b:");
    let c = read_int("Enter c:");

    let aa = a * a;
    let bb = b * b;
    let cc = c * c;

    if aa == bb + cc || bb == aa + cc || cc == aa + bb {
        println!("YES");
    } else {
        println!("NO");
    }
}