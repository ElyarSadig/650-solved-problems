package main

import (
	"fmt"
	"os/exec"
)

func main() {
	cmd := exec.Command("calc.exe")

	err := cmd.Start()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	
	err = cmd.Wait()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	fmt.Println("Calculator executed successfully")
}