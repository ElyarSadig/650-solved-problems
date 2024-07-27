package main

import (
	"fmt"
	"runtime"
)

func main() {
	fmt.Println("Count CPU is", runtime.GOMAXPROCS(0))
}