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
// Updated at Mon Apr  6 02:05:56 UTC 2026
