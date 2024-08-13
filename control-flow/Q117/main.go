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
		if sign == 1 && i != n {
			fmt.Printf("%d + ", i)
		} else if i != n {
			fmt.Printf("%d - ", i)
		} else {
			fmt.Printf("%d = ", i)
		}
		sum += sign * i
		sign *= -1
	}
	fmt.Printf("%d", sum)
}
