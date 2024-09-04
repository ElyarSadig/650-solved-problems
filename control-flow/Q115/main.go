package main

import "fmt"

func main() {
	var (
		account         int
		beginingBalance float64
		totalCharge     float64
		totalCredits    float64
		creditLimit     float64
	)
	for {
		fmt.Print("Enter account number(-1 to end): ")
		fmt.Scan(&account)
		if account == -1 {
			break
		}
		fmt.Print("Enter begining balance: ")
		fmt.Scan(&beginingBalance)
		fmt.Print("Enter total charges: ")
		fmt.Scan(&totalCharge)
		fmt.Print("Enter total credits: ")
		fmt.Scan(&totalCredits)
		fmt.Print("Enter credit limit: ")
		fmt.Scan(&creditLimit)
		if beginingBalance + totalCredits - totalCharge > creditLimit {
			fmt.Println("Account:", account)
			fmt.Println("Credit Limit:", creditLimit)
			fmt.Println("lance:", beginingBalance + totalCredits - totalCharge)
			fmt.Println("Credit Limit Exceeded")
		}
	}
}
