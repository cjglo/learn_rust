pub fn run()
{
    // I will show some vector code next to array to see difference

    // Vectors are resizable arrays. declared this way: 
    // Arrays say: 
    // let mut numbers: [i32; 4] = [1, 2, 3, 4];
        let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[0] = 10; // Compilter will throw error if mut varibale never changed, so I change it here.

    println!("{}", numbers.len());

    numbers.push(11);
    println!("{:?}", numbers);
}