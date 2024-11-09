use std::io;

fn main() {
    let mut input = String::new();
    let v: f64;
    let T: f64;

    println!("Input wind speed in kilometer/houre: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    v = input.trim().parse().expect("invalid number for v");

    input.clear();

    println!("Input air temperature in degrees Celsius: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    T = input.trim().parse().expect("invalid number for T");

    let wci = 13.12 + 0.6215 * T - 11.37 * v.powf(0.16) + 0.3965 * T * v.powf(0.16);
    println!("The wind chil index is: {}", wci as i32)
}
