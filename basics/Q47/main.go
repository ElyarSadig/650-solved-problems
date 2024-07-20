package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println(os.Environ())
	fmt.Println("*****************************")
	fmt.Println("PATH is", os.Getenv("PATH"))
}
