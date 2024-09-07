package main

import "fmt"

func main() {
	var (
		sign int = 1
		x    float64
		n    int
		sum  float64
		pow  float64 = 1
		fact float64 = 1
	)
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	for i := 1; i <= n; i++ {
		fact *= float64(i)
		pow *= x
		if sign == 1 {
			fmt.Print(" +", pow, "/", fact)
		} else {
			fmt.Print(" -", pow, "/", fact)
		}
		sign = -sign
		sum += pow / fact * float64(sign)
	}
	fmt.Print(" = ", sum)
}
