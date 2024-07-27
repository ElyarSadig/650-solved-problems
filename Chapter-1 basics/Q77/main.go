package main

import "fmt"

func main() {
	var (
		a int
		b float64
		c string
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	fmt.Print("Enter c: ")
	fmt.Scan(&c)
	fmt.Printf("Type a is %T\n", a)
	fmt.Printf("Type b is %T\n", b)
	fmt.Printf("Type c is %T\n", c)
}