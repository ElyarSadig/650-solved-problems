package main

import "fmt"

func main() {
	var x float64
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	y := 1.0 / (x*x + x + 1)
	fmt.Println("y =", y)
}
