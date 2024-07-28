package main

import "fmt"

func main() {
	var Kg float64
	fmt.Print("Enter Kg: ")
	fmt.Scan(&Kg)
	g := Kg * 1000
	fmt.Println("Weight(g) =", g)
}