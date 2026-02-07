use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut it = input.split_whitespace();

    let mut score1: i32 = 0;
    let mut score2: i32 = 0;

    for _ in 0..5 {
        println!("Player 1 Please Enter 1:(scissors), 2:(stone) 3:(paper):");
        let player1: i32 = it
            .next()
            .expect("Missing input for player1")
            .parse()
            .expect("Not a valid number");

        println!("Player 2 Please Enter 1:(scissors), 2:(stone) 3:(paper):");
        let player2: i32 = it
            .next()
            .expect("Missing input for player2")
            .parse()
            .expect("Not a valid number");

        if (player1 == 1 && player2 == 3)
            || (player1 == 3 && player2 == 2)
            || (player1 == 2 && player2 == 1)
        {
            score1 += 1;
        } else if (player2 == 1 && player1 == 3)
            || (player2 == 3 && player1 == 2)
            || (player2 == 2 && player1 == 1)
        {
            score2 += 1;
        }
    }

    println!("-----------------------------------------");
    println!("Score for Player1: {}", score1);
    println!("Score for Player2: {}", score2);
}