package main

import "fmt"

func main() {
	var n int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	var num int
	var sum int
	for i := 1; i <= n; i++ {
		fmt.Print("Enter a number: ")
		fmt.Scan(&num)
		sum += num
	}
	fmt.Println("Sum is", sum)
	fmt.Println("Average is", float64(sum)/float64(n))
}
