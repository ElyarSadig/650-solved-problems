package main

import "fmt"

func main() {
	var count int
	var price int
	fmt.Print("Enter Count: ")
	fmt.Scan(&count)
	fmt.Print("Enter Price: ")
	fmt.Scan(&price)
	var sells = price * count
	fmt.Println("Sells is", sells)
}