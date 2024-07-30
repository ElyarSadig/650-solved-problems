package main

import (
	"fmt"
	"time"
)

func main() {
	var (
		byy int
		bmm int
		bdd int
	)
	fmt.Print("Enter birth date(year): ")
	fmt.Scan(&byy)
	fmt.Print("Enter birth date(month): ")
	fmt.Scan(&bmm)
	fmt.Print("Enter birth date(day): ")
	fmt.Scan(&bdd)
	cyy := time.Now().Year()
	cmm := time.Now().Month()
	cdd := time.Now().Day()
	if cdd < bdd {
		cmm -= 1
		cdd += 30
	}
	day := cdd - bdd
	if int(cmm) < bmm {
		cyy -= 1
		cmm += 12
	}
	month := int(cmm) - bmm
	year := cyy - byy
	days := day + month*30 + year*365
	hh := days * 24
	mm := hh * 60
	ss := mm * 60
	fmt.Printf("Old is: %d/%d/%d\n", year, month, day)
	fmt.Printf("Hour is (hh:mm:ss) (%d:%d:%d)", hh, mm, ss)
}
