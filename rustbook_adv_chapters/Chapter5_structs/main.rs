// This covers only the ending material, in the intro folder I have 
// some of the early projects from before chapter 5
fn main()
{


println!();

// --------- Update syntax for instances:

struct User
{
    username: String,
    password: f32,
    active: bool,
}

impl User
{
    fn new(username: String, password: f32, active: bool) -> User
    {
        User
        {
            username,
            password,
            active,
        }
    }
}


// Update shorthand syntax here:

let user1 = User::new(String::from("jsmith"), 23.25, true);

let user2 = User {
    username: String::from("akolo"),
    ..user1 // HERE IT IS, the .. implies all remaining fields are the same as user1
};

println!("user1: {} {} {}, user2: {} {} {}", user1.username, user1.password, user1.active, user2.username, user2.password, user2.active);






// -------- Tuple structs

// These are helpful when want a named tuple, but a whole struct with defined fields is unecessary

struct Color(i32, i32, i32);

let black = Color(0, 0, 0);

println!("{}", black.1);




// ------- Debugging println!

// Add a header to get a struct to use {:?}
#[derive(Debug)]
struct Rectangle
{
    height: u8,
    width: u8,
}

let rect1 = Rectangle {
    height: 10,
    width: 8,
};

println!("\n{:?}", rect1);  // This now spits out all fields for us to see

// end

}