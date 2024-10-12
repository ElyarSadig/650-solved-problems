package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var s string
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("Enter a sentence: ")
	
	if scanner.Scan() {
		s = scanner.Text()
	}
	
	var count int
	for _, ch := range s {
		if ch >= '0' && ch <= '9' {
			count++
		}
	}
	
	fmt.Println("Count is", count)
}
