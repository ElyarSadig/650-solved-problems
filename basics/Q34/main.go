package main

import "fmt"

func main() {
	var (
		liter float64
		mile  float64
	)
	fmt.Print("Enter liter: ")
	fmt.Scan(&liter)
	fmt.Print("Enter mile: ")
	fmt.Scan(&mile)
	result := (mile / liter) * 0.265179
	fmt.Println("miles / gallons is", result)
}
