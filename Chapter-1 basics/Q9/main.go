package main

import "fmt"

func main() {
	var salary float64
	fmt.Print("Enter Salary: ")
	fmt.Scan(&salary)
	b := salary * 0.7
	m := salary * 0.1
	p := salary - b - m
	fmt.Printf("Salary: %f b = %f m = %f p = %f", salary, b, m, p)
}
