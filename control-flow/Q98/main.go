package main

import "fmt"

func main() {
	var (
		i int
		x int
		y int
	)
	fmt.Print("Enter x: ")
	fmt.Scan(&x)
	fmt.Print("Enter y: ")
	fmt.Scan(&y)
	var temp int
	var r int
	if x > y {
		temp = x
		r = y
	} else {
		temp = y
		r = x
	}
	for temp >= r {
		temp -= r
		i++
	}
	if x > y {
		fmt.Printf("%d / %d = %d", x, y, i)
	} else {
		fmt.Printf("%d / %d = %d", x, y, i)
	}
}
