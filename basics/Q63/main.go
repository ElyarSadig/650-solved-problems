package main

import (
	"fmt"
)

func main() {
	var r rune
	fmt.Print("Enter a char: ")
	fmt.Scanf("%c", &r)
	fmt.Println(r)
}
