// Enum example with IP addresses

fn main()
{

    
    enum IpAddrType // Can create enum and attatch another type with a ()
    {
        V4(IpAddr),
        V6(IpAddr),
    }
    
    struct IpAddr // Defined the above is possible
    {
        //something
    }

    



    // Can even have varying types with lots of diff characteristics

    enum Message // This has a noral type, a anonymous struct with two intgers, a type with a string, and a type with a tuple
    { // The real advatage of this rather then building each as own struct: We can now build functions easily that would take any of these as parameters
        Quit,
        Move(x: i32, y: i32),
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    



    // Can also define impls on our enums!
    impl Message 
    {
        fn call(&self)
        {
            // method body......
        }
    }


 






    // OPTION Starts here ------------------------------------------ very helpful

    let some_num = Some(5);
    let some_string = Some("a string");

    let absent_num: Option<i32> = None; // Must include type since can't be inferred


    let x = 3; 

    // This below will not compile! It is how rust makes sure you handle Option cases with both Some and None
    // let sum = x + some_num;

    // Option is what you can use besides nullptr

    












    // Other topics herer:

    // Matching and "if let" both here

    // check chapter for details, both are straight forward

    // Only complex part of match is "binding" which just works with Enums with an (attahced data type), letting you use that 
    // extra data type in arm

}