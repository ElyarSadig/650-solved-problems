package main

import "fmt"

func main() {
	for i := 1; i <= 8; i++ {
		for j := 8 - i; j >= 0; j-- {
			fmt.Print(i, " ")
		}
		fmt.Println()
	}
}
