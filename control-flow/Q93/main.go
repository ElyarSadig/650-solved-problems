package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var s string
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("Enter a string: ")
	scanner.Scan()
	s = scanner.Text()
	var count int
	runes := []rune("aeouiAEOUI")
	for _, r := range s {
		for _, v := range runes {
			if v == r {
				count++
			}
		}
	}
	fmt.Println("Count is", count)
}
