class Greeter:
    def __init__(self, name):
        self.name = name

    def greet(self):
        print(f"Hello, World from Python OOP! (Greetings to {self.name})")

if __name__ == "__main__":
    greeter = Greeter("GitHub Automation")
    greeter.greet()
