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
	newNum := (1 << n) | num
	fmt.Println("Number before setting", n, "bit:", num, "(in decimal)")
	fmt.Println("Number after setting", n, "bit:", newNum, "(in decimal)")
}
