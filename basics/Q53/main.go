package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("This is the name/path of the script:")
	fmt.Println("Number of arguments:", len(os.Args))
	fmt.Println("Argument list:", os.Args)
}
