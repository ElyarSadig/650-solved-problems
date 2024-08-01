package main

import "fmt"

func main() {
	var n int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	var num int
	for i:=0;i < n + 1; i++ {
		fmt.Print("Enter num: ")
		fmt.Scan(&num)
		if num % 9 == 0 {
			fmt.Println(num)
		}
	}
}