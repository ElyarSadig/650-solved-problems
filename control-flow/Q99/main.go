package main

import "fmt"

func main() {
	var n int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	fact := 1
	sum := 0.0
	for i := 1; i < n; i++ {
		fact *= i
		sum += 1.0 / float64(fact)
	}
	fmt.Println("Sum is:", sum)
}
