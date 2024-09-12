package main

import "fmt"

func main() {
	var n int
	fmt.Print("Enter an odd number: ")
	fmt.Scan(&n)
	y := n / 3
	k := n + 3
	for i := 1; i < y+2; i++ {
		fmt.Print(" ")
	}
	for i := 1; i < y+1; i++ {
		fmt.Print("*")
	}
	fmt.Println()
	for i := 1; i < y+1; i++ {
		for j := 1; j < y-i+1; j++ {
			fmt.Print(" ")
		}
		fmt.Print("*")
		for c := n + 2; c < y+k+1; c++ {
			fmt.Print(" ")
		}
		if i == n {
			fmt.Print(" ")
		} else {
			fmt.Print("*")
		}
		fmt.Println()
		k += 2
	}
	for po := 1; po < n/2+1; po++ {
		fmt.Print("*")
		for pow := 1; pow < n+1; pow++ {
			fmt.Print(" ")
		}
		fmt.Println("*")
	}
	m := 1
	for i := 1; i < y+1; i++ {
		for c := y + 2; c < y+m+1; c++ {
			fmt.Print(" ")
		}
		fmt.Print(" ")
		if i == y {
			fmt.Print(" ")
			for h := 1; h < y+1; h++ {
				fmt.Print("*")
			}
		} else {
			fmt.Print("*")
		}
		for j := 1; j < 2*y-2*i+y+1; j++ {
			fmt.Print(" ")
		}
		if i == y {
			fmt.Print("")
		} else {
			fmt.Print("*")
		}
		fmt.Println()
		m += 1
	}
}
