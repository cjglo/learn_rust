pub fn run()
{
    // Basic set up:

    let mut x = 10;
    let xr = &x;

    println!("x is {}", xr); // Notice this does the same as passing x

    // changing values
    // Need to assign a mutable reference
    let y = &mut x;
    *y += 1; // remeber to deref like C++
    println!("x is {}", y);
    
    // NOTE: If printed above but with x AN ERROR IS THROWN!
    // This is because you can't have mutable reference then have an immut vrrow
    // println borrows it, so to be able to use x, need to wrap that mut reference
    // in a code block
    let mut z = 5;

    // Now the mut reference in code block, no longer exists afterward
    {
        let zz = &mut z;
        *zz -= 1;
    }

    println!("z is {}", z);

    // The rule is: Many immutable references
    // OR 1 mutuable reference, not both
}