package main

import "fmt"

func main() {
	var n int
	var sum int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	for i := 1; i <= n; i++ {
		if i < n {
			fmt.Print(i, " + ")
		} else {
			fmt.Print(i, " = ")
		}
		sum += i
	}
	fmt.Print(sum)
}
