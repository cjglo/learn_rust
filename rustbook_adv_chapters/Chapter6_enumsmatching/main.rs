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









    // Option Starts here ------------------------------------------ very helpful

    

}