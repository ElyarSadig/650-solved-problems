package main

import "fmt"

func main() {
	payment := 75000
	var tedad int
	fmt.Print("Enter tedad: ")
	fmt.Scan(&tedad)
	extera := float64(payment * 12 * tedad) * 13.5 / 100
	fmt.Printf("Extera is: %10.6f", extera)
}