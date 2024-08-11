package main

import "fmt"

func main() {
	var (
		a int
		b int
		c int
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	fmt.Print("Enter c: ")
	fmt.Scan(&c)
	if a*a == b*b + c*c || b*b == a*a + c*c || c*c == a*a + b*b {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}