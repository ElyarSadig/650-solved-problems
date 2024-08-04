package main

import "fmt"

func main() {
	var ch rune
	for {
		fmt.Print("Enter a char: ")
		fmt.Scanf("%c\n", &ch)
		switch ch {
		case 'b', 'B':
			fmt.Println("You selected Lady")
		case 'd', 'D':
			fmt.Println("You selected Miss")
		case 'p', 'P':
			fmt.Println("You selected Professor")
		case 'a', 'A':
			fmt.Println("You selected Mr")
		case 'j', 'J':
			fmt.Println("You selected Excellency")
		case 'm', 'M':
			fmt.Println("You selected Wife")
		default:
			return
		}
	}
}
