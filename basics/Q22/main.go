package main

import "fmt"

func main() {
	var height float64
	var base float64
	fmt.Print("Enter Height: ")
	fmt.Scan(&height)
	fmt.Print("Enter base: ")
	fmt.Scan(&base)
	area := height * base * 0.5
	fmt.Println("Area is", area)
}