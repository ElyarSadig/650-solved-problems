package main

import "fmt"

func main() {
	var salary float64
	fmt.Print("Enter Salary: ")
	fmt.Scan(&salary)
	reward := salary * 0.15
	fmt.Println("Reward is:", reward)
}