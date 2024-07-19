package main

import "fmt"

func main() {
	var year float64
	fmt.Print("Enter your age: ")
	fmt.Scan(&year)
	minute := year * 365.25 * 24 * 60
	fmt.Println("Minute is", minute)
}