package main

import "fmt"

func main() {
	var x int
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	m := (x << 5) - x
	n := -((x << 4) + x)
	y := m + n + 5
	fmt.Println("y =", y)
}
