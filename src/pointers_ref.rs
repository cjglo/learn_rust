pub fn run()
{
    // One big thing to note with Rust is non-primitive variables get "moved" when assigned
    // So if assign one vector to another, the original is deleted, so references needed

    // To be continued!!! This is sort of a test ground for some things for now

     // Primitive Array
  let mut arr1 = [1, 2, 3];
  let mut arr2 = arr1;

  arr1[0] = 15;
  arr2[1] = 23;

  println!("{:?}", arr2);
  println!("{:?}", arr1);



  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, vec2));
}