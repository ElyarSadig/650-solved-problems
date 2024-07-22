package main

import "fmt"

func main() {
	s1 := 13 * 16
	fmt.Println("s1 =", s1)
	s2 := 2 * 3
	fmt.Println("s2 =", s2)
	b := s1 / s2
	a := s1 % s2
	fmt.Println("b =", b)
	fmt.Println("a =", a)
}
