// Long file covering Rust-from-C++ Git Lessons, broken into units


fn main() 
{

    // Control FLOW:
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

// Ends Controll Flow section

println!("\n\n\n\n");

// Begins: Primitive Data Types & operations:

let x: bool = true;
// As above, you decalre var type with : and type after
// u and i are short hand for unsigned and signed varaibles
let y = 34i8; // unsigned 34
let z: u8 = 34u8; // 34 unsgined 8 declared after as well
let alpha = 34f64; // compiler infers type

println!("{}, {}, {}, {}", x, y, z, alpha);

// Key Note: "let" lets you redeclare variables, which is very helpful since they are immutable by default
let b = 34;
println!("b: {}", b);
let b: bool = false;
println!("b: {} \n", b);


// You can also use binary, octal, hex, : using 0b 0o 0x
// You can also use _ and they will be ignored

let x = 0b1010;
let y = 0o_15;
let z = 0xff;

println!("{}, {}, {}", x, y, z);

// Strings and Chars are unicode in Rust, so slightly different then C++
// Will come back to this later



}