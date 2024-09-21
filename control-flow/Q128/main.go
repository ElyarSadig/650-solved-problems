package main

import "fmt"

func main() {
	for i := 8; i >= 5; i-- {
		for j := 8; j >= 5; j-- {
			for k := 8; k >= 5; k-- {
				for m := 8; m >= 5; m-- {
					if i == j || i == k || i == m || j == k || j == m || k == m {
						continue
					} else {
						fmt.Println(i*1000 + j*100 + k*10 + m)
					}
				}
			}
		}
	}
}
