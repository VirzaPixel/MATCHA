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
// Updated at Sat May  9 03:25:55 UTC 2026
