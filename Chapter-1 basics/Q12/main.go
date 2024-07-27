package main

import "fmt"

func main() {
	var num int
	fmt.Print("Enter number: ")
	fmt.Scan(&num)
	n1 := num % 10
	n2 := num / 10
	fmt.Println("Reverse is", n1*10+n2, "\tSum is", n1+n2)
}
