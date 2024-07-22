package main

import (
	"fmt"
	"math"
)

func main() {
	var (
		x1 float64
		x2 float64
		y1 float64
		y2 float64
	)
	fmt.Print("Enter x1: ")
	fmt.Scan(&x1)
	fmt.Print("Enter x2: ")
	fmt.Scan(&x2)
	fmt.Print("Enter y1: ")
	fmt.Scan(&y1)
	fmt.Print("Enter y2: ")
	fmt.Scan(&y2)
	dist := math.Sqrt(math.Pow((x2-x1), 2) + math.Pow((y2-y1), 2))
	fmt.Println("Distance is:", dist)
}
