use std::env;

fn main() {
    let vars: Vec<String> = env::vars().map(|(k, v)| format!("{}={}", k, v)).collect();
    println!("{:?}", vars);
}