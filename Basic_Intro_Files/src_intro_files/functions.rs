pub fn run()
{
    greeting("Hello", "Jane"); // Note: Not mentioned, but it seems the compiler looks for fn (so no prototype needed)

    println!("{}", add(2, 4));

    // Can also do Closures:
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; 
    // NOTE: Not only nice and compact, but also can use outside variables, n3 would be out of scope normally
    println!("{}", add_nums(3, 4));

}


fn greeting(greet: &str, name: &str)
{
    println!("{} {}, nice to meet you", greet, name);
}

// This function adds two nums, the "->" marks the return
fn add(n1: i32, n2: i32) -> i32
{
    // NOTICE NO SEMICOLON! This tells compiler this is what we want to return
    n1 + n2
}

