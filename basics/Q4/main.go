package main

import (
	"fmt"
	"math"
)

func main() {
	var nSides int
	var sLenght float64

	fmt.Print("Input number of sides: ")
	fmt.Scan(&nSides)
	fmt.Print("Input the lenght of a slide: ")
	fmt.Scan(&sLenght)

	pArea := (float64(nSides) * (sLenght * sLenght)) / (4 + math.Tanh(math.Pi/float64(nSides)))
	fmt.Println("The area of the polygon is:", pArea)
}
