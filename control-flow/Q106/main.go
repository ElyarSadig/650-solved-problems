package main

import "fmt"

func main() {
	f1 := 0
	f2 := 1
	f3 := 1
	var num int
	fmt.Print("Enter a number: ")
	fmt.Scan(&num)
	for f3 < num {
		f3 = f1 + f2
		f1 = f2
		f2 = f3
	}
	if f3 == num {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
