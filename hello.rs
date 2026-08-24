struct Greeter;

impl Greeter {
    fn greet(&self) {
        println!("Hello World");
    }
}

fn main() {
    let greeter = Greeter;
    greeter.greet();
}
// Updated at Mon Aug 24 01:26:29 UTC 2026
