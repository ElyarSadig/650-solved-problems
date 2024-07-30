package main

import "fmt"

func main() {
	var (
		n  int
		id int
		h  int
		hp int
	)
	fmt.Print("Enter n: ")
	fmt.Scan(&n)
	for i := 1; i <= n; i++ {
		fmt.Print("Enter id: ")
		fmt.Scan(&id)
		fmt.Print("Enter h: ")
		fmt.Scan(&h)
		fmt.Print("Enter hp: ")
		fmt.Scan(&hp)
		ov := 0.0
		if h > 40 {
			ov = 0.5 * float64((h-40)*hp)
		}
		p := ov + float64(hp * h)
		fmt.Println("id =", id, " ov =", ov, " p =", p)
	}
}
