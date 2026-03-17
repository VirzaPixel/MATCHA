struct Greeter {
    name: String,
}

impl Greeter {
    fn new(name: &str) -> Greeter {
        Greeter {
            name: name.to_string(),
        }
    }

    fn greet(&self) {
        println!("Hello, World from Rust OOP! (Greetings to {})", self.name);
    }
}

fn main() {
    let greeter = Greeter::new("GitHub Automation");
    greeter.greet();
}
