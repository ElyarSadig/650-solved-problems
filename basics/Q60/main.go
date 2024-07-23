package main

import (
	"fmt"
	"reflect"
)

type User struct {
	ID        int
	FirstName string
	LastName  string
}

func main() {
	var user User = User{
		ID:        1,
		FirstName: "Elyar",
		LastName:  "Sadig",
	}
	var a1 int = 10
	var a2 int64 = 10
	var s string = "Hello World"
	sliceOfBytes := make([]byte, 1024)
	var r rune = 'A'
	var array [10]int

	fmt.Println("Memory Size of a1 is", getSize(a1), "bytes")
	fmt.Println("Memory Size of a2 is", getSize(a2), "bytes")
	fmt.Println("Memory Size of s is", getSize(s), "bytes")
	fmt.Println("Memory Size of user is", getSize(user), "bytes")
	fmt.Println("Memory Size of sliceOfBytes is", getSize(sliceOfBytes), "bytes")
	fmt.Println("Memory Size of r is", getSize(r), "bytes")
	fmt.Println("Memory Size of array is", getSize(array), "bytes")
}

func getSize(input any) uintptr {
	t := reflect.TypeOf(input)
	return t.Size()
}
