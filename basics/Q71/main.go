package main

import "fmt"

func main() {
	var num int
	fmt.Print("Enter a number: ")
	fmt.Scan(&num)
	flippedNumber := ^num
	fmt.Println("Original Number =", num)
	fmt.Println("Number after bits are flipped =", flippedNumber)
}