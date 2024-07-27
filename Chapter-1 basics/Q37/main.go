package main

import (
	"fmt"
	"math"
)

func main() {
	var r float64
	fmt.Print("Enter r: ")
	fmt.Scan(&r)
	n := 2 * r
	p := math.Pi * n
	a := math.Pi * r * r
	fmt.Printf("n = %f\np = %f\na = %f", n, p, a)
}
