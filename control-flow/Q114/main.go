package main

import "fmt"

func main() {
	var (
		quiz1   float64
		quiz2   float64
		midTerm float64
		final   float64
	)
	fmt.Print("Enter grade for quiz one: ")
	fmt.Scan(&quiz1)
	fmt.Print("Enter grade for quiz two: ")
	fmt.Scan(&quiz2)
	fmt.Print("Enter grade for mid term: ")
	fmt.Scan(&midTerm)
	fmt.Print("Enter grade for final: ")
	fmt.Scan(&final)
	grade := (quiz1 + quiz2) * 5 * 0.25 + midTerm * 0.25 + final * 0.5
	switch {
	case grade >= 90:
		fmt.Println("Grade is A")
	case grade >= 80:
		fmt.Println("Grade is B")
	case grade >= 70:
		fmt.Println("Grade is C")
	case grade >= 60:
		fmt.Println("Grade is D")
	default:
		fmt.Println("Grade is E")
	}
}