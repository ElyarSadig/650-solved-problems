package main

import (
	"fmt"
)

func main() {
	var (
		id1  int     = -1
		id2  int     = -1
		max1 float64 = -1
		max2 float64 = -1
		n    int
	)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	if n < 2 {
		fmt.Println("Please enter a number greater than 1")
	} else {
		var id int
		var avg float64
		for range n {
			fmt.Print("Enter id: ")
			fmt.Scan(&id)
			fmt.Print("Enter average: ")
			fmt.Scan(&avg)
			if avg > max1 {
				max2 = max1
				max1 = avg
				id2 = id1
				id1 = id
			} else {
				if avg > max2 {
					id2 = id
					max2 = avg
				}
			}
		}
		fmt.Println("Max2 =", max2, "\t", "Id2 =", id2)
	}
}
