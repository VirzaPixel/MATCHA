<?php

class Greeter {
    private $name;

    public function __construct($name) {
        this->name = $name;
    }

    public function greet() {
        echo "Hello, World from PHP OOP! (Greetings to " . $this->name . ")\n";
    }
}

$greeter = new Greeter("GitHub Automation");
$greeter->greet();
?>
