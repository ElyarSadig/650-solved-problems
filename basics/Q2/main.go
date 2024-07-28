package main

import "fmt"

func main() {
	const PI float64 = 22.0 / 7
	var height float64
	var radius float64

	fmt.Print("Height of cylinder: ")
	fmt.Scanln(&height)
	fmt.Print("Radius of cylinder: ")
	fmt.Scanln(&radius)

	volume := PI * radius * radius * height
	surArea := ((2 * PI * radius) * height) + (2 * (PI * radius * radius))
	fmt.Println("Volume is:", volume)
	fmt.Println("Surface Area is:", surArea)
}
