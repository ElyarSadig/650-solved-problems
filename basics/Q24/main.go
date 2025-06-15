package main

import "fmt"

func main() {
	var (
		k float64
		x float64
		n float64
	)
	fmt.Print("Enter k: ")
	fmt.Scan(&k)
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	acceleration := (k - x) * 60 / n
	fmt.Println("acceleration is", acceleration)
}
