package main

import (
	"fmt"
	"math"
)

func main() {
	var degree float64
	fmt.Print("Enter degree: ")
	fmt.Scan(&degree)
	radian := degree * (math.Pi/180)
	fmt.Println("Radian is:", radian)
}