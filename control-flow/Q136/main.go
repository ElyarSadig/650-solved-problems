package main

import "fmt"

func main() {
	for i := 100; i <= 500; i += 50 {
		for j:=i; j<=i+500; j += 100 {
			fmt.Printf("%d  ", j)
		}
		fmt.Println()
	}
}
