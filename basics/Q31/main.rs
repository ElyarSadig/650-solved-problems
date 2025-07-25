fn main() {
    println!("Number\tSquare\tCube");
    for i in 0..=10 {
        println!("{}\t{}\t{}", i, i * i, i * i * i);
    }
}
