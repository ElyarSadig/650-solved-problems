package main

import "fmt"

func main() {
	var (
		a int
		b string
	)
	fmt.Print("Enter integer for a: ")
	fmt.Scan(&a)
	fmt.Print("Enter a name for b: ")
	fmt.Scan(&b)
	fmt.Printf("Pointer a is %p\n", &a)
	fmt.Printf("Pointer b is %p", &b)
}