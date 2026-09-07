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
// Updated at Mon Sep  7 03:37:57 UTC 2026
