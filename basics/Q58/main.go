package main

import "fmt"

func main() {
	var (
		a float64
		b float64
		c float64
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	fmt.Print("Enter c: ")
	fmt.Scan(&c)
	x := (b - c) / (3 - a)
	y := 3 * x + c
	fmt.Printf("(%f, %f)", x, y)
}