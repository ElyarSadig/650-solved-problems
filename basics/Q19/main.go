package main

import "fmt"

func main() {
	var (
		R1 float64
		R2 float64
		R3 float64
	)
	fmt.Print("Enter R1:")
	fmt.Scan(&R1)
	fmt.Print("Enter R2:")
	fmt.Scan(&R2)
	fmt.Print("Enter R3:")
	fmt.Scan(&R3)

	R := 1.0/R1 + 1.0/R2 + 1.0/R3
	fmt.Println("R =", 1/R)
}
