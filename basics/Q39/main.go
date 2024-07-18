package main

import "fmt"

func main() {
	var (
		m int
		n int
	)
	fmt.Print("Enter m: ")
	fmt.Scan(&m)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	a := m*m - n*n
	b := 2 * m * n
	c := m*m + n*n
	fmt.Println("a is", a)
	fmt.Println("b is", b)
	fmt.Println("c is", c)
}