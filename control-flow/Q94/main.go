package main

import "fmt"

func main() {
	for i := 7; i > 0; i-- {
		for j := i; j > 0; j-- {
			fmt.Print(j, " ")
		}
		fmt.Println()
	}
}
