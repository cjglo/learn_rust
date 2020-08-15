pub fn run()
{
    // Like C++, arrays are fixed length

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);  // once again, use debug (:?) to get eveyrthing, in this case the entire array

    // Important note: has to be exact on size, assignment would not work if defined 4 elements

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Using mut you can make it possible to change value in array
    // It would be say way as most languages: numbers[2] = 20;

    // Arrays are stack allocated, this can show total memory by passing reference
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    // common to type "use std::mem" to shorten, this would then be just mem::size_of_val(etc)

    
    // Getting a slice
    let slice: &[i32] = &numbers[0..2]; // This gives just first two, index start at 0 until 2
    println!("Slice {:?}", slice);
}