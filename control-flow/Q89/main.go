package main

import "fmt"

func main() {
	var num int
	fmt.Print("Enter a number: ")
	fmt.Scan(&num)
	pow := 10
	temp := num
	reversedNum := 0
	for temp > 0 {
		reversedNum = reversedNum*pow + temp%10
		temp /= 10
	}
	if reversedNum == num {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
