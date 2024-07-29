package main

import (
	"fmt"
)

func main() {
	f1 := 1
	f2 := 1
	var n int
	fmt.Print("Enter a number: ")
	fmt.Scan(&n)
	if n == 1 {
		fmt.Println(f1)
	} else if n == 2 {
		fmt.Println(f1)
		fmt.Println(f2)
	} else {
		fmt.Println(f1)
		fmt.Println(f2)
		var f3 int
		for i := 3; i <= n; i++ {
			f3 = f1 + f2
			fmt.Println(f3)
			f1 = f2
			f2 = f3
		}
	}
}
