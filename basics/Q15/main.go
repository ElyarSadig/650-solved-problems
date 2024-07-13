package main

import "fmt"

func main() {
	var n int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	n1 := n
	n2 := n1*10 + n
	n3 := n2*10 + n
	fmt.Printf("%d + %d + %d = %d", n1, n2, n3, n1 + n2 + n3)
}
