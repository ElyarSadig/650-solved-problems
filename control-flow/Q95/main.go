package main

import "fmt"

func main() {
	var (
		score1  int
		score2  int
		player1 int
		player2 int
	)
	for range 5 {
		fmt.Print("Player 1 Please Enter 1:(scissors), 2:(stone) 3:(paper): ")
		fmt.Scan(&player1)
		fmt.Print("Player 2 Please Enter 1:(scissors), 2:(stone) 3:(paper): ")
		fmt.Scan(&player2)
		if (player1 == 1 && player2 == 3) || (player1== 3 && player2 == 2) || (player1 == 2 && player2 == 1) {
			score1++
		} else if (player2 == 1 && player1 == 3) || (player2== 3 && player1 == 2) || (player2 == 2 && player1 == 1) {
			score2++
		}
	}
	fmt.Println("-----------------------------------------")
	fmt.Println("Score for Player1:", score1)
	fmt.Println("Score for Player2:", score2)
}
