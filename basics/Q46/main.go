package main

import (
	"fmt"
)

func main() {
	const M_PER_MILE float64 = 1609.35
	const M_PER_FOOT float64 = 0.30480
	var (
		miles int
		feet  int
	)
	fmt.Print("Enter the number of miles: ")
	fmt.Scan(&miles)
	fmt.Print("Enter the number of feet: ")
	fmt.Scan(&feet)
	totalMeters := float64(miles)*M_PER_MILE + float64(feet)*M_PER_FOOT
	totalKiloMeters := totalMeters / 1000
	kilometers := int(totalKiloMeters)
	meters := (totalKiloMeters - float64(kilometers)) * 1000
	fmt.Println("The distance is", kilometers, "kilometers,", meters, "meters.")
}
