package main

import "fmt"

func main() {
	var (
		x int
		y int
		z int
	)
	fmt.Print("Enter first number: ")
	fmt.Scan(&x)
	fmt.Print("Enter second number: ")
	fmt.Scan(&y)
	fmt.Print("Enter third number: ")
	fmt.Scan(&z)

	a1 := min(x, y, z)
	a3 := max(x, y, z)
	a2 := (x + y + z) - a1 - a3
	fmt.Println("Numbers in sorted order:", a1, a2, a3)
}
