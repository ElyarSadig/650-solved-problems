package main

import "fmt"

func main() {
	for i := 1; i <= 10; i++ {
		for j := 1; j <= 5; j++ {
			for k := 1; k <= 2; k++ {
				if i*1000+j*2000+k*5000 == 10000 {
					fmt.Printf("1000 * %d + 2000 * %d + 5000 * %d = %d \n", i, j, k, 10000)
				}
			}
		}
	}
}
