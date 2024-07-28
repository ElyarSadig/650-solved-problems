package main

import (
	"fmt"
	"os"
)

func main() {
	hostName, err := os.Hostname()
	if err != nil {
		fmt.Println("Error is", err)
		return
	}
	fmt.Println("HostName:", hostName)
}