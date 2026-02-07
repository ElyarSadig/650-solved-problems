use std::thread;

fn main() {
    let count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    println!("Count CPU is {}", count);
}