package main

import "fmt"

func main() {
	fmt.Println("Result is:")
	for i := 1000; i <= 1100; i++ {
		if i%9 == 0 {
			fmt.Print(i, " ")
		}
	}
}