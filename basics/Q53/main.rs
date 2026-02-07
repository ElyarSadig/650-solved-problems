use std::env;

fn main() {
    println!("This is the name/path of the script:");
    let args: Vec<String> = env::args().collect();
    println!("Number of arguments: {}", args.len());
    println!("Argument list: {:?}", args);
}