package main

import "fmt"

func main() {
	var (
		a float64
		b float64
		h float64
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	fmt.Print("Enter h: ")
	fmt.Scan(&h)
	area := h * (a + b) / 2
	fmt.Println("Area is", area)
}