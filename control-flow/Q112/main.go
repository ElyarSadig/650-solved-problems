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
	if a > (b+c) || c > (b+a) || b > (c+a) {
		fmt.Println("NO")
	} else {
		fmt.Println("YES")
	}
}
