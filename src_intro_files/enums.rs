// Enums are types with a few definite values

enum Movement 
{
    // In here are Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement)
{
    // preform actiondepdning on info
    // Use match, which is like a switch statement
    match m
    {
        Movement::Up => println!("Avatar Up"),
        Movement::Down => println!("Avatar Down"),
        Movement::Left => println!("Avatar Left"),
        Movement::Right => println!("Avatar Right")
    }
}


pub fn run()
{
 let avatar1 = Movement::Left;
 let avatar2 = Movement::Up;

 move_avatar(avatar1);
 move_avatar(avatar2);
}