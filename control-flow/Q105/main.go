package main

import "fmt"

func main() {
	var cost float64
	fmt.Print("Enter cost: ")
	fmt.Scan(&cost)
	for i := 0; i < 10; i++ {
		cost -= cost * 20 / 100
		fmt.Printf("cost for year %d: %f\n", i+1, cost)
	}
}
