package main

import "fmt"

func main() {
	var (
		a int
		b int
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	fmt.Printf("a + b = %d\n", a+b)
	fmt.Printf("a - b = %d\n", a-b)
	fmt.Printf("a * b = %d\n", a*b)
	fmt.Printf("a / b = %d\n", a/b)
	fmt.Printf("a^2 + b^2 = %d\n", a*a+b*b)
	fmt.Printf("a^3 + b^3 = %d", a*a*a+b*b*b)
}
