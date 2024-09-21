package main

import "fmt"

func main() {
	t := true
	f := false
	fmt.Println("Logical AND &&")
	fmt.Printf("%t && %t: %d\n", t, t, boolToInt(t && t))
	fmt.Printf("%t && %t: %d\n", t, f, boolToInt(t && f))
	fmt.Printf("%t && %t: %d\n", f, t, boolToInt(f && t))
	fmt.Printf("%t && %t: %d\n", f, f, boolToInt(f && f))
	fmt.Println("Logical OR ||")
	fmt.Printf("%t || %t: %d\n", t, t, boolToInt(t || t))
	fmt.Printf("%t || %t: %d\n", t, f, boolToInt(t || f))
	fmt.Printf("%t || %t: %d\n", f, t, boolToInt(f || t))
	fmt.Printf("%t || %t: %d\n", f, f, boolToInt(f || f))
	fmt.Println("Logical NOT !")
	fmt.Printf("!%t: %d\n", t, boolToInt(t))
	fmt.Printf("!%t: %d\n", f, boolToInt(f))
}

func boolToInt(v bool) int {
	if v {
		return 1
	}
	return 0
}