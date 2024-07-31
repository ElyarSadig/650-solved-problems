package main

import "fmt"

func main() {
	var (
		x int
		y int
	)
	for {
		fmt.Print("Enter x: ")
		fmt.Scan(&x)
		fmt.Print("Enter y: ")
		fmt.Scan(&y)
		if x == 0 && y == 0 {
			break
		}
		sum := 0
		temp := y
		if y < 0 {
			temp = -y
		}
		for i := 1; i < temp+1; i++ {
			sum += x
		}
		if y < 0 {
			sum = -sum
		}
		fmt.Println(x, "*", y, "=", sum)
	}
}
