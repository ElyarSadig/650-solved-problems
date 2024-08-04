package main

import "fmt"

func main() {
	for i := 1; i <= 8; i++ {
		for j := 1; j <= 8; j++ {
			fmt.Print(i, " ")
		}
		fmt.Println()
	}
}