package main

import "fmt"

func main() {
	var (
		sum  float64
		sign = 1
		fact = 1.0
		n    int
	)
	fmt.Print("Enter a number: ")
	fmt.Scan(&n)
	for i := 1; i < n+1; i++ {
		fact *= float64(i)
		if sign == 1 {
			fmt.Print(" +", i, "/", fact)
		} else {
			fmt.Print(" -", i, "/", fact)
		}
		sum += float64(sign*i) / fact
		sign = -sign
	}
	fmt.Println(" =", sum)
}
