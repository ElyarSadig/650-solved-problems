package main

import "fmt"

func main() {
	var (
		h float64
		r float64
	)
	fmt.Print("Enter hour: ")
	fmt.Scan(&h)
	fmt.Print("Enter rate: ")
	fmt.Scan(&r)
	tp := h * r
	t := tp / 10
	pp := tp - t
	fmt.Println("Total Payment:", tp)
	fmt.Println("Tax: ", t)
	fmt.Println("Payment: ", pp) 
}