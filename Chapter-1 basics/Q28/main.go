package main

import (
	"fmt"
	"runtime"
)

func main() {
	fmt.Printf("runtime.GOOS: %v\n", runtime.GOOS)
}