package main

import (
	"fmt"
)

func main() {
	var (
		x int
		y int
	)
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	fmt.Print("Enter y: ")
	fmt.Scan(&y)
	z := x*x*x + 2*x*x + 3*y - 5
	fmt.Println("z is", z)
}
