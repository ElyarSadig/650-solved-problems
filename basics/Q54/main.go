package main

import "fmt"

func main() {
	var (
		p float64
		h float64
	)
	fmt.Print("Enter p: ")
	fmt.Scan(&p)
	fmt.Print("Enter h: ")
	fmt.Scan(&h)
	s := p * h / 2
	fmt.Println("S is", s)
}