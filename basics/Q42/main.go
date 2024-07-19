package main

import "fmt"

func main() {
	var pages int
	var Gb int
	fmt.Print("Enter pages: ")
	fmt.Scan(&pages)
	fmt.Print("Enter size of memory: ")
	fmt.Scan(&Gb)
	bookBytes := 80 * 30 * pages
	number := float64(1024*1024*1024*Gb) / float64(bookBytes)
	fmt.Println("Number of book is", number)
}
