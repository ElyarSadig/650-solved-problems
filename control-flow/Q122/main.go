package main

import "fmt"

func main() {
	var (
		a int
		b int
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	if a % b == 0 || b % a == 0 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}