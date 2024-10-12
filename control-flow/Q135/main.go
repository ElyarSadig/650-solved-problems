package main

import "fmt"

func main() {
	for i := 2; i <= 14; i += 4 {
		temp := i
		for j := 1; j <= 6; j++ {
			fmt.Printf("%d\t", temp)
			temp += 2
		}
		fmt.Println()
	}
}