package main

import "fmt"

func main() {
	var (
		num int
		n   int
	)
	fmt.Print("Enter a number: ")
	fmt.Scan(&num)
	fmt.Print("Enter nth bit to check (0-31): ")
	fmt.Scan(&n)
	bitStatus := (num >> n) & 1
	fmt.Println("The", n, "bit is set to", bitStatus)
}
