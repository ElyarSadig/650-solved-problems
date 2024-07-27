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
	newNum := num ^ (1 << n)
	fmt.Println("Number before toggling", n, "bit:", num)
	fmt.Println("Number after toggling", n, "bit:", newNum)
}
