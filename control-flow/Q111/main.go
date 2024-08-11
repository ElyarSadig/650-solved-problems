package main

import "fmt"

func main() {
	sign := -1.0
	var n int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	p := 4.0
	k := 3.0
	fmt.Println("i    \tPI")
	fmt.Println("===   \t===================")
	for i := 1; i <= n; i++ {
		p += sign * 4 / k
		k += 2
		sign *= -1
		fmt.Println(i, "\t", p)
	}
}
