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
