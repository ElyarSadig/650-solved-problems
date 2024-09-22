package main

import "fmt"

func main() {
	nums := [4]int{1, 2, 4, 9}
	count := 0
	for _, i := range nums {
		for _, j := range nums {
			for _, c := range nums {
				for _, k := range nums {
					if i == j || i == c || i == k || j == c || j == k || c == k {
						continue
					} else {
						fmt.Println(i*1000 + j*100 + c*10 + k)
						count++
					}
				}
			}
		}
	}
	fmt.Println("Count:", count)
}
