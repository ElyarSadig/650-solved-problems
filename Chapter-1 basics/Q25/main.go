package main

import "fmt"

func main() {
	var (
		consumption float64
		capacity    float64
	)
	fmt.Print("Enter Consumption: ")
	fmt.Scan(&consumption)
	fmt.Print("Enter Capacity: ")
	fmt.Scan(&capacity)
	dist := capacity / consumption * 100
	fmt.Println("Distance is", dist)
}