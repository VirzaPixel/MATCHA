#include <iostream>

class Greeter {
public:
    void greet() {
        std::cout << "Hello World" << std::endl;
    }
};

int main() {
    Greeter greeter;
    greeter.greet();
    return 0;
}
