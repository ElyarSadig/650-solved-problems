package main

import "fmt"

func main() {
	var (
		I float64
		R float64
	)
	fmt.Print("Enter I: ")
	fmt.Scan(&I)
	fmt.Print("Enter R: ")
	fmt.Scan(&R)
	fmt.Println("V =", I * R)
}