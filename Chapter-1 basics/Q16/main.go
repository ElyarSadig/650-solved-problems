package main

import (
	"fmt"
	"math"
)

func main() {
	var (
		amt   int
		rate  float64
		years int
	)
	fmt.Print("Enter amount: ")
	fmt.Scan(&amt)
	fmt.Print("Enter rate: ")
	fmt.Scan(&rate)
	fmt.Print("Enter years: ")
	fmt.Scan(&years)
	futureValue := float64(amt) * (math.Pow((1 + 0.01*rate), float64(years)))
	fmt.Printf("Future Value is %f", futureValue)
}
