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
// Updated at Sun Jul 12 03:17:03 UTC 2026
