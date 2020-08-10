pub fn run()
{
// TWO TYPES OF STRINGS:
// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

let hello = "Hello"; // This is primitive by default
// To signify the second type:
let hello2 = String::from("Hello");
println!("{} {}", hello, hello2);

// There are two ways to add to string, "push" for a char and "push str" for strings
// REMEBER: have to add mut to push to it
let mut hello3 = String::from("Hello ");
hello3.push('Q');

println!("{}", hello3);

// Paused at 41:47
}