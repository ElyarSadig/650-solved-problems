package main

import "fmt"

func main() {
	var (
		n      int
		base6  int
		base1  int
		totalW int
		totalM int
		id     int
		date   int
		m_w    int
		sumM   float64
		sumW   float64
		pay    float64
	)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	for i := 0; i < n; i++ {
		fmt.Print("Enter an id: ")
		fmt.Scan(&id)
		fmt.Print("Enter a date: ")
		fmt.Scan(&date)
		fmt.Print("Enter a m_w(0|1): ")
		fmt.Scan(&m_w)
		fmt.Print("Enter base: ")
		fmt.Scan(&base1)
		fmt.Print("Enter payment: ")
		fmt.Scan(&pay)
		if base1 == 6 {
			base6 += 1
		}
		if m_w == 0 {
			sumW += pay
			totalW += 1
		} else {
			sumM += pay
			totalM += 1
		}
	}
	fmt.Println("********* Result ********* \n Number of base six is", base6)
	fmt.Println("Number of women is", totalW)
	if totalW > 0 {
		fmt.Println("Average(woman) pay is", sumW/float64(totalW))
	}
	if totalM > 0 {
		fmt.Println("Average(man) pay is", sumM/float64(totalM))
	}
}
