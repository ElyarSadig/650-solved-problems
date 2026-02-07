use std::io;

fn main() {
    let _ = io::stdin();

    for i in 0..3 {
        for j in 0..6 {
            for k in 0..11 {
                for l in 0..201 {
                    let sum = i * 500 + j * 200 + k * 100 + l * 50;
                    if sum == 1000 {
                        println!("({}, {}, {}, {})", i, j, k, l);
                    }
                }
            }
        }
    }
}