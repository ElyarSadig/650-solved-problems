package main

import "fmt"

func main() {
	var s string
	fmt.Print("Enter a string: ")
	fmt.Scan(&s)
	for _, r := range s {
		fmt.Print(string(r) + " ")
	}
}