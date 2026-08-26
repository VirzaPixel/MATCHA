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
// Updated at Wed Aug 26 01:26:41 UTC 2026
