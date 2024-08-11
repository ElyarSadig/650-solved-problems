package main

import "fmt"

func main() {
	var (
		sumMiles   float64
		sumGallons float64
		gallon     float64
		mile       float64
	)
	for {
		fmt.Print("Enter the gallons used (-1 to end): ")
		fmt.Scan(&gallon)
		if gallon == -1 {
			break
		}
		fmt.Print("Enter the miles driven: ")
		fmt.Scan(&mile)
		rate := mile / gallon
		fmt.Println("The miles / gallons for this tank was", rate)
		sumGallons += gallon
		sumMiles += mile
	}
	average := sumMiles / sumGallons
	fmt.Println("The overall average miles / gallons was", average)
}
