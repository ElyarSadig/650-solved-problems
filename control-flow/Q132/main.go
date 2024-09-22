package main

import "fmt"

func main() {
	for i := 1; i <= 3; i++ {
		for j := 1; j <= 3; j++ {
			for k := 1; k <= 3; k++ {
				fmt.Printf("%d\t", i*100+j*10+k)
			}
			fmt.Println()
		}
	}
}
