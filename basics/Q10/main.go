package main

import "fmt"

func main() {
	var y1 int
	var y2 int
	fmt.Print("Enter Price for first year: ")
	fmt.Scan(&y1)
	fmt.Print("Enter Price for second year: ")
	fmt.Scan(&y2)
	t := float64(y2-y1) / float64(y1)
	y3 := float64(y1) + float64(y2)*t
	fmt.Printf("Extera: %.1f \t\tPrice next year = %.2f", t, y3)
}