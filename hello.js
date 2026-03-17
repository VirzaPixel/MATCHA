class Greeter {
    constructor(name) {
        this.name = name;
    }

    greet() {
        console.log(`Hello, World from JavaScript OOP! (Greetings to ${this.name})`);
    }
}

const greeter = new Greeter("GitHub Automation");
greeter.greet();
