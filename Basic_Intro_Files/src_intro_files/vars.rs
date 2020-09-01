// A lot like c++
// Biggest change is variables are not mutable by default

pub fn run()
{
    let name = "Brad";

    println!("My name is {}", name);

    let mut age = 18;
    println!("My age is {}", age);
    age = 19;
    println!("My age is {}", age);

    // Do have const, which needs to be assigned type.
}