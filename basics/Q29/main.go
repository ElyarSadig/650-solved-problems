package main

import (
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	cmd := exec.Command("go", "list", "all")
	out, err := cmd.Output()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	packages := strings.Split(string(out), "\n")

	for _, pkg := range packages {
		if pkg != "" {
			fmt.Println(pkg)
		}
	}
}
