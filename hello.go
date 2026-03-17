package main

import "fmt"

type Greeter struct {
    Name string
}

func (g Greeter) Greet() {
    fmt.Printf("Hello, World from Go OOP! (Greetings to %s)\n", g.Name)
}

func main() {
    greeter := Greeter{Name: "GitHub Automation"}
    greeter.Greet()
}
