package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {
	code := `import sys
print(sys.copyright)
`
	tmpfile, err := os.CreateTemp("", "script*.py")
	if err != nil {
		fmt.Println("Error creating temporary file:", err)
		return
	}
	defer os.Remove(tmpfile.Name())

	if _, err := tmpfile.Write([]byte(code)); err != nil {
		fmt.Println("Error writing to temporary file:", err)
		return
	}
	if err := tmpfile.Close(); err != nil {
		fmt.Println("Error closing temporary file:", err)
		return
	}

	cmd := exec.Command("python", tmpfile.Name())
	output, err := cmd.CombinedOutput()
	if err != nil {
		fmt.Println("Error executing Python script:", err)
		return
	}

	fmt.Println(string(output))
}
