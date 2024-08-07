package main

import "fmt"

func main() {
	var N int
	fmt.Print("Enter N: ")
	fmt.Scan(&N)
	for i := 1; i <= N; i++ {
		fmt.Printf("%d\t%d\t%d\t%d\n", i, i*10, i*100, i*1000)
	}
}