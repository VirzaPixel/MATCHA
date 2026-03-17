#include <iostream>
#include <string>

class Greeter {
private:
    std::string name;

public:
    Greeter(std::string n) : name(n) {}

    void greet() {
        std::cout << "Hello, World from C++ OOP! (Greetings to " << name << ")" << std::endl;
    }
};

int main() {
    Greeter greeter("GitHub Automation");
    greeter.greet();
    return 0;
}
