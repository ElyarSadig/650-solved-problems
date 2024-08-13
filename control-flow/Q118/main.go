package main

import "fmt"

func main() {
	var (
		n    int
		sign int = -1
		sum  int
	)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	for i := 1; i <= n; i++ {
		if sign == 1 {
			fmt.Print(" + ", i)
		} else {
			fmt.Print(" - ", i)
		}
		sum += sign * i
		sign = -sign
	}
	fmt.Print(" = ", sum)
}
