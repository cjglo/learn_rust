fn main()
{
    // Practice of string ownership


    {    
        let s1 = String::from("hello"); // s1 is owner

        take_owner(s1); // s1 no longer valid, now passed value is owner and goes out of scope after

        let s2 = String::from("world");

        let _s3 = take_owner(s2);  // s2 is owner, ownership passed to value, then value passed to s3 which is no owner


    } // s3 out of scope

    

     pub fn take_owner(word: String) -> String
     {
        word
     }
     println!("Complete!");





     


     // Now just gonna try multiple returns and delcarations with tuple return

     let val = String::from("testing");

     let (val2, num) = tuple_return(val);

     println!("The word was {} and the secret num was {}", val2, num);


     // function for tuple return
     pub fn tuple_return(word: String) -> (String, u32)
     {
        println!("Inside function, wow!");

        (word, 23)
     }

}