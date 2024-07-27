package main

import "fmt"

func main() {
	var base float64
	var height float64

	fmt.Print("Enter base: ")
	fmt.Scanln(&base)
	fmt.Print("Enter height: ")
	fmt.Scanln(&height)
	
	area := base * height

	fmt.Println("Area is:", area)
}