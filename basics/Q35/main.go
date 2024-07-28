package main

import "fmt"

func main() {
	var x int
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	fmt.Printf("%d ^ 2 = %d\n%d ^ 3 = %d", x, x*x, x, x*x*x)
}