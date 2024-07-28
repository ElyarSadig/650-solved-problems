package main

import "fmt"

func main() {
	var n int
	fmt.Print("Enter a number between 10000 to 99999: ")
	fmt.Scan(&n)
	temp := n
	r5 := temp % 10
	temp /= 10
	r4 := temp % 10
	temp /= 10
	r3 := temp % 10
	temp /= 10
	r2 := temp % 10
	temp /= 10
	r1 := temp % 10
	fmt.Printf("Result is %d   %d   %d   %d   %d", r1, r2, r3, r4, r5)
}
