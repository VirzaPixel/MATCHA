class Greeter
  def initialize(name)
    @name = name
  end

  def greet
    puts "Hello, World from Ruby OOP! (Greetings to #{@name})"
  end
end

greeter = Greeter.new("GitHub Automation")
greeter.greet
