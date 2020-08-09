pub fn run()
{
    // Any variable or int must use {}
    println!("Number: {}", 1);
    // Basic Formating
    println!("{} is from {}", "Brad", "New Jersey");
    // Positional Arguements Used
    println!("{0} is from {1} and {0} is on the {1} soccer team.", "Brad", "New Jersey");


    println!("{name} likes to play {activity} on {day}", day = "Tuesday", name = "John", activity = "football");
}