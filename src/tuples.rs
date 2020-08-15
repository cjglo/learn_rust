pub fn run()
{
    // Tuples group together values of different types, Max is 12

    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}