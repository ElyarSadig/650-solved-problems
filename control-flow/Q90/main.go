package main

import "fmt"

func main() {
	var (
		m int
		n int
	)
	fmt.Print("Enter m: ")
	fmt.Scan(&m)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	temp := m
	for i := 0; i < n-1; i++ {
		sum := 0
		for j := 0; j < temp; j++ {
			sum = sum + m
		}
		temp = sum
	}
	fmt.Printf("%d ^ %d = %d", m, n, temp)
}
