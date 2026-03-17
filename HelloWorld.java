public class HelloWorld {
    private String name;

    public HelloWorld(String name) {
        this.name = name;
    }

    public void greet() {
        System.out.println("Hello, World from Java OOP! (Greetings to " + this.name + ")");
    }

    public static void main(String[] args) {
        HelloWorld greeter = new HelloWorld("GitHub Automation");
        greeter.greet();
    }
}
