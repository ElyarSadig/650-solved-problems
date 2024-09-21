package main

import "fmt"

func main() {
	multiply := 1
	var n int
	fmt.Print("Enter a number: ")
	fmt.Scan(&n)
	fmt.Println("Number\t\tDigit\t\tMultiply")
	fmt.Println("======\t\t======\t\t======")
	var digit int
	for ; n > 0; n /= 10 {
		digit = n % 10
		if digit%2 == 1 {
			multiply *= digit
		}
		fmt.Printf("%d\t\t%d\t\t%d\n", n, digit, multiply)
	}
	fmt.Println("Multiply is", multiply)
}
