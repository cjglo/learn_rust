// Quick little program to practice some new syntax for if statements and match statements


fn main() 
{
let person1_age = 23;
let person2_age = 45;


fn num_of_drinkers(person1_age: i32, person2_age: i32) -> i32
{
    if person1_age <= 21 && person2_age <= 21  
    { 
        0
    } 
    else if person1_age >= 21 && person2_age >= 21 
    { 
        2 
    }
    else 
    {
        1
    }
}

println!("{}", num_of_drinkers(person1_age, person2_age));





let mut counter = 0;
// No do-while so need loop
loop 
{
    println!("Go Lions");
    if counter == 0 { break; }
    counter -= 1;
}



// Match/Switch Statement

fn bouncer(person: i32) 
{
    match person
    {
        person if person < 21 => { println!("Sorry, no entry."); }
        21 | 22 => { println!("Welcome happy birthday youngin' "); }
        person if person > 22 => { println!("Welcome"); }
        _ => { println!(" Welcome youngin' "); }
    }
}

bouncer(person1_age);





}