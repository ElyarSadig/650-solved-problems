package main

import (
	"fmt"
	"strings"
)

func main() {
	var inputString string
	var repeat int
	fmt.Print("Enter a string: ")
	fmt.Scan(&inputString)
	fmt.Print("Enter repeat: ")
	fmt.Scan(&repeat)
	
	s := strings.Builder{}
	for i := 0; i < repeat; i++ {
		s.WriteString(inputString)
	}
	fmt.Println(s.String())
}