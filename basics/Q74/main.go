package main

import (
	"fmt"
	"time"
)

func main() {
	var n int
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	now := time.Now()
	newTime := now.Add(time.Second * time.Duration(n))
	fmt.Println("Current Time:", now)
	fmt.Println("New Time:", newTime)
}
