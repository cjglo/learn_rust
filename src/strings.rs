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

// Create string with capacity:
let mut s = String::with_capacity(10);
s.push('a');

println!("{}", s);

// Assertion testing - VERY HELPFUL!!!!
assert_eq!(1, s.len());  // Nothing will happen because length is one
// Had this been wrong in throws an error and stops main
}