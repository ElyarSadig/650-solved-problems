package main

import "fmt"

func main() {
	var str string
	fmt.Print("Enter a string: ")
	fmt.Scan(&str)
	var count int
	for range str {
		count++
	}
	fmt.Println("Count is", count)
}