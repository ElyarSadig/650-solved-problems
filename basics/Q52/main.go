package main

import "fmt"

func main() {
	var (
		x int
		y int
	)
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	fmt.Print("Enter y: ")
	fmt.Scan(&y)
	x = x + y
	y = x - y
	x = x - y
	fmt.Println("X =", x, "Y =", y)
}
