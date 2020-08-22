pub fn run()
{
    // Inifite loop:
    let mut count = 0;

    loop 
    {
        count += 1;
        println!("Number: {}", count);

        // this will go on forever, so need to write condition to break
        if count == 21 { break; }
    }
    // ^^^ Could be wrong, but I assume the above is used as a do-while since it would excute once
    // Obviosuly to make it act like a do-while there would need to be some edits


    count = 0;
    
    // While loop
    while count <= 100 
    {
        if count % 6 == 0
        {
            println!("{}", count);
        }
        count += 1;
    }


    // For Loop
    for x in 0..100  // Remember that 100 is not included
    {
        if x % 10 == 0
        {
            println!("{}", x);
        }
    }

}