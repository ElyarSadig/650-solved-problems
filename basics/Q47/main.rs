use std::env;

fn main() {
    let vars: Vec<String> = env::vars().map(|(k, v)| format!("{}={}", k, v)).collect();
    println!("{:?}", vars);
    println!("*****************************");
    let path = env::var("PATH").unwrap_or_else(|_| String::new());
    println!("PATH is {}", path);
}