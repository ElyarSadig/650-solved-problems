package main

import "fmt"

func main() {
	var (
		num1 int
		num2 int
	)
	fmt.Print("Enter num1: ")
	fmt.Scan(&num1)
	fmt.Print("Enter num2: ")
	fmt.Scan(&num2)
	num1 ^= num2
	num2 ^= num1
	num1 ^= num2
	fmt.Println("Num1 after swapping =", num1)
	fmt.Println("Num2 after swapping =", num2)
}