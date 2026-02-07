use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Not a valid number");

    let pow: i32 = 10;
    let mut temp = num;
    let mut reversed_num: i32 = 0;

    while temp > 0 {
        reversed_num = reversed_num * pow + (temp % 10);
        temp /= 10;
    }

    if reversed_num == num {
        println!("YES");
    } else {
        println!("NO");
    }
}