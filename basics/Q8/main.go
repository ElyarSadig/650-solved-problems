package main

import "fmt"

func main() {
	var w float64
	fmt.Print("Input w: ")
	fmt.Scan(&w)
	m := 3e-23
	l := 950
	tedad := (w * float64(l)) / m
	fmt.Println("Tedad: ", tedad)
}
