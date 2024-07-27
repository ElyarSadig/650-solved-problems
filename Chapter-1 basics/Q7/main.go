package main

import "fmt"

func main() {
	var real float64
	var img float64

	fmt.Print("Enter real part: ")
	fmt.Scan(&real)
	fmt.Print("Enter image part: ")
	fmt.Scan(&img)

	c := complex(real, img)
	fmt.Println("complex:", c)
}