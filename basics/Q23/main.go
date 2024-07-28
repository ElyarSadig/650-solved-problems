package main

import "fmt"

func main() {
	var (
		a4  int
		pen int
		t   float64
	)
	fmt.Print("Enter A4 price: ")
	fmt.Scan(&a4)
	fmt.Print("Enter pen price: ")
	fmt.Scan(&pen)
	fmt.Print("Enter t: ")
	fmt.Scan(&t)

	cost := 50*float64(pen)*t/100 + 150*float64(a4)*t/100
	fmt.Println("Extra cost is", cost)
}
