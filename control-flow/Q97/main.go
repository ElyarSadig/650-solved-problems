package main

import "fmt"

func main() {
	var ch rune

outerloop:
	for {
		fmt.Print("Enter a char: ")
		fmt.Scanf("%c\n", &ch)
		switch ch {
		case 'w', 'W':
			fmt.Println("You love white color")
		case 'r', 'R':
			fmt.Println("You love Red color")
		case 'y', 'Y':
			fmt.Println("You love Yellow color")
		case 'b', 'B':
			fmt.Println("You love Blue color")
		case 'g', 'G':
			fmt.Println("You love Green color")
		case 'e', 'E':
			break outerloop
		default:
			fmt.Println("No color chosen")
		}
	}
}
