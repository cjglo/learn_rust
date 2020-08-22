pub fn run()
{
 
/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Default is "i32"
let x = 1;

// Default is "f64"
let y = 2.5;

// To get a value to be a specific value, you must but semicolon and then one of the keys
let z:i64 = 98743234;

// Find max size
println!("Max i32: {}", std::i32::MAX);
println!("Max i64: {}", std::i64::MAX);

// One thing to help see everything easily is the debug symbol for print {:?}
println!("{:?}", (x, y, z));
}