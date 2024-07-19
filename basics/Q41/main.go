package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	exePath, err := os.Executable()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	absPath, err := filepath.Abs(exePath)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	fmt.Println("Current File Name", absPath)
}
