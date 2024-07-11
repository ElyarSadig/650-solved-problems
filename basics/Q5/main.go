package main

import (
	"fmt"
	"math"
)

func main() {
	var (
		v float64
		T float64
	)
	fmt.Print("Input wind speed in kilometer/houre: ")
	fmt.Scan(&v)
	fmt.Print("Input air temperature in degrees Celsius: ")
	fmt.Scan(&T)
	wci := 13.12 + 0.6215*T - 11.37 * math.Pow(v, 0.16) + 0.3965 * T * math.Pow(v, 0.16)
	fmt.Println("The wind chil index is:", int(wci))
}