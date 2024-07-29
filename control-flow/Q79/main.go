package main

import "fmt"

func main() {
	var num int
	var yes string
	for {
		fmt.Print("Enter a number: ")
		fmt.Scan(&num)
		sum := 0
		i := 1
		for i < num {
			if num%i == 0 {
				sum += i
			}
			i++
		}
		if sum == num {
			fmt.Println("Perfected")
		} else {
			fmt.Println("Not Perfect")
		}
		fmt.Print("Continue ? ")
		fmt.Scan(&yes)
		if yes[0] == 'N' || yes[0] == 'n' {
			break
		}
	}
}
