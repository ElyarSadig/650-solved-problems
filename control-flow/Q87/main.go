package main

import "fmt"

func main() {
	var x float64
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	sign := 1
	pow := 1.0
	sum := 0.0
	sum1 := 0.0
	for i := 1; i <= 10; i++ {
		pow = pow * x
		sum1 = sum1 + float64(i) * pow
		sum = sum + float64(sign) * 1.0 / sum1
		sign = -sign
	}
	fmt.Println("S =", sum)
}