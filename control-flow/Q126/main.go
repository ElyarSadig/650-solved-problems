package main

import "fmt"

func main() {
	var month int
	for {
		fmt.Print("Enter month: ")
		fmt.Scan(&month)
		switch month {
		case 1:
			fmt.Println("Farvardin")
		case 2:
			fmt.Println("Ordibehesht")
		case 3:
			fmt.Println("Khordad")
		case 4:
			fmt.Println("Tir")
		case 5:
			fmt.Println("Mordad")
		case 6:
			fmt.Println("Shahrivar")
		case 7:
			fmt.Println("Mehr")
		case 8:
			fmt.Println("Aban")
		case 9:
			fmt.Println("Azar")
		case 10:
			fmt.Println("Dey")
		case 11:
			fmt.Println("Bahman")
		case 12:
			fmt.Println("Esfand")
		default:
			return
		}
	}
}
