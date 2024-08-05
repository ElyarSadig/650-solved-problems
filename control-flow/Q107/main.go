package main

import "fmt"

func main() {
	for i := range 3 {
		for j := range 6 {
			for k := range 11 {
				for l := range 201 {
					sum := i*500 + j*200 + k*100 + l*50
					if sum == 1000 {
						fmt.Printf("(%d, %d, %d, %d)\n", i, j, k, l)
					}
				}
			}
		}
	}
}
