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
// Updated at Sun Sep 20 04:04:21 UTC 2026
