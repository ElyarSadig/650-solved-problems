package main

import (
	"fmt"
	"math"
)

func main() {
	var radian float64
	fmt.Print("Enter Radian: ")
	fmt.Scan(&radian)
	degree := radian * (180 / math.Pi)
	fmt.Println("Degree is", degree)
}
