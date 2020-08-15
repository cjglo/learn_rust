pub fn run()
{
    // getting into the habbit of writing datat typp=es
    let age: u8 = 18;
    let check_id: bool = false;

    // Simple if/ if else/ else
    if age >= 21 && check_id
    {
        println!("Bartender: What would you like? ");
    }
    else if age < 21 && check_id
    {
        println!("Bartender: Sorry, you have to leave.");
    }
    else
    {
        println!("Bartender: I'll need to see your ID.")
    }

    // NOTE: Or is || 
    // like C++
    let knows_person: bool = true;

    if knows_person || check_id
    {
        println!("Bartender knows age.");
    }
}