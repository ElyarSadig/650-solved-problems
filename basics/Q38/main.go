package main

import "fmt"

func main() {
	var year float64
	fmt.Print("Enter year: ")
	fmt.Scan(&year)
	day := 365.25 * year
	fmt.Println("Day is", day)
	month := int(day / 30)
	fmt.Println("Month is", month)
	seconds := day * 24 * 3600
	fmt.Println("Seconds is", seconds)
}
