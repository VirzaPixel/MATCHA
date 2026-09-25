package main

import "fmt"

type Greeter struct{}

func (g Greeter) Greet() {
    fmt.Println("Hello World")
}

func main() {
    greeter := Greeter{}
    greeter.Greet()
}
// Updated at Fri Sep 25 04:05:10 UTC 2026
