package main

import "fmt"

func main() {
	result := 0
	var (
		a  int
		ch string
		b  int
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter op: ")
	fmt.Scan(&ch)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	switch ch {
	case "+", "a", "A":
		result = a + b
	case "-", "S", "s":
		result = a - b
	case "*", "X", "x":
		result = a * b
	case "/", "D", "d":
		result = a / b
	case "%", "M", "m":
		result = a % b
	default:
		fmt.Println("Invalid operator")
		return
	}
	fmt.Printf("%d %s %d = %d", a, ch, b, result)
}
