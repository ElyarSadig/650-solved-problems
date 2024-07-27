package main

import "fmt"

func main() {
	var (
		a int
		b int
	)
	fmt.Print("Enter a: ")
	fmt.Scan(&a)
	fmt.Print("Enter b: ")
	fmt.Scan(&b)
	fmt.Printf("%d + %d = %d\n", a, b, a+b)
	fmt.Printf("%d - %d = %d\n", a, b, a-b)
	fmt.Printf("%d * %d = %d\n", a, b, a*b)
	fmt.Printf("%d / %d = %f\n", a, b, float64(a)/float64(b))
	fmt.Printf("%d %% %d = %d", a, b, a%b)
}
