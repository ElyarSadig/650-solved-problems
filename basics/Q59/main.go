package main

import (
	"fmt"
	"math"
)

func main() {
	var (
		a float64
		b float64
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	y := a * math.Sin(b)
	fmt.Println("y is", y)
}