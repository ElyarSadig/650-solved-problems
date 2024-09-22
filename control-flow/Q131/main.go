package main

import "fmt"

func main() {
	countOdd := 0
	countEven := 0
	sumOdd := 0
	sumEven := 0
	var n int
	fmt.Print("Enter a number: ")
	fmt.Scan(&n)
	fmt.Println("Number\tDigit")
	fmt.Println("======\t=====")
	for n > 0 {
		digit := n % 10
		fmt.Printf("%d\t%d\n", n, digit)
		if digit%2 == 0 {
			countEven++
			sumEven += digit
		} else {
			countOdd++
			sumOdd += digit
		}
		n /= 10
	}
	fmt.Println("Average Even is:", float64(sumEven)/float64(countEven))
	fmt.Println("Average Odd is:", float64(sumOdd)/float64(countOdd))
}
