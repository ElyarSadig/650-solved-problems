package main

import (
	"fmt"
)

func main() {
	const PI float64 = 22.0 / 7
	var radius float64
	fmt.Print("Radius of sphere: ")
	fmt.Scan(&radius)

	volume := (4.0 / 3) * PI * radius * radius * radius
	surArea := 4 * PI * radius * radius
	fmt.Println("Surface Area is:", surArea)
	fmt.Println("Volume is:", volume)
}
