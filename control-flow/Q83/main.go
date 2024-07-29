package main

import "fmt"

func main() {
	var (
		m     float64
		tedad int
		s     float64
		ansi  string
	)
	for {
		fmt.Print("Enter m: ")
		fmt.Scan(&m)
		fmt.Print("Enter tedad: ")
		fmt.Scan(&tedad)
		fmt.Print("Enter s: ")
		fmt.Scan(&s)
		k := m + (m * (float64(tedad) + 1)*s) / 2400
		p := k / float64(tedad)
		fmt.Println("k =", k, "\t", p)
		fmt.Print("Do you want to continue(y/n): ")
		fmt.Scan(&ansi)
		if ansi[0] == 'n' {
			break
		}
	}
}
