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
	newNum := (^(1 << n)) & num
	fmt.Println("Number before clearing", n, "bit:", num)
	fmt.Println("Number after clearing", n, "bit:", newNum)
}
