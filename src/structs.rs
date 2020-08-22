// Structs are used to create custom data types

// struct Color 
// {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person
{
    first_name: String,
    last_name: String
}

impl Person
{
    // Constructor:
    fn new(first: &str, last: &str) -> Person
    {
        Person
        {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get Function
    // Note: self is like keyword this
    fn full_name(&self) -> String
    {
        // Format is like print, but doesn't print
        // also again, notice no semicolon because we will return this value
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Function
    // mut added because it will be changed! So need to add that 
    fn set_last_name(&mut self, last: &str)
    {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String)
    {
        // Parathesis builds tuple, so build and return in same line
        (self.first_name, self.last_name) // No semicolon because it is returning
    }
}




pub fn run()
{
    // let mut c = Color
    // {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // println!("Color: {} {} {}", c.red, c.green, c.blue)


    // let mut c = Color(255, 0, 0);

    // c.0 = 200;

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Mary", "Doe");  // Constrcuts
    println!("{} {}", p.first_name, p.last_name); // Prints by attributes

    p.set_last_name("Williams");  // sets new value
    println!("Person {}", p.full_name()); // calls func to print


}