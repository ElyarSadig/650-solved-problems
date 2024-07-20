package main

import (
	"fmt"
	"os/user"
)

func main() {
	user, err := user.Current()
	if err != nil {
		fmt.Println("Error:",err)
		return
	}
	fmt.Println("Username is:", user.Username)
}
