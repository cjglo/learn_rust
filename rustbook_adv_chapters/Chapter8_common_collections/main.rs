

fn main()
{
    // 1
    // VECTORS -----------------------------

    let v: Vec<i32> = Vec::new();

    // vec! macro helpful for creating vectors (also since value supplied, type is inferred and so no type declaration needed)
    let v = vec![1, 2, 3, 4, 5];

    // Terms for adding is .push() 

    // Retrieving an element has the usual two methods.  [i] or .get(i)
    // Key to remember is [i] will cause program to panic if ref outside of vector
    // get(i) will return a None and therefore be fine

    // Another note is borrowing mentioned before, but vectors can show example
    let mut v = vec![1, 2, 3];

    let first = &v[0];

    // If next line is left uncommented, this will cause error:
    //
    // v.push(6);

    // This is because vec uses mut references to create a new vec (inner array) each push, so this would be two mut references!



    // Iteration -------------------- (with vectors)

    for i in &mut v // delete mut and use just & to iterate over const ref
    {
        *i += 50;  // Obviosuly, use * to deref
    }



    // Storing diff values -----------

    // Nothing much to add, but remember to create Enum that holds values and then can do this (since vec would hold enum types)




    // 2
    // STRING --------------------------------------

    let s = "initial contents".to_string();
    let mut s2 = String::from("Other string"); // mut is unrelated to this, its for next thing

    s2.push_str(", now even longer!");

    println!("{}...{}", s, s2);


    // adding strings
    s2 = String::from(" Other Contents");
    let s3 = s + &s2; // Note: s now moved so gone / Also: s2 is ref cause add function is (self, &str)

    println!("{}", s3);

    // format macro
    let s1 = String::from("kit");
    let s2 = String::from("kat");

    let s3 = format!("{}-{}", s1, s2); // Note: format doesn't take ownership so all still allowed

    println!("{}", s3);

    // Can't index a string because Rust uses UTF so one character may be many bytes
    // best way to iterate over a string is to use .chars()
    for c in "word".chars()
    {
        println!("{}", c);
    }

    // .bytes() returns each byte, if that is needed





    // 3
    // HASH MAPS -----------------------------------------

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 50);

    // can also do it from tuples if tuple has key and value
    // It is a little complicated so look it up if doing it



    // Retrieving 
    let team_name = String::from("Blue");
    let current_score = scores.get(&team_name); // Returns Some(&50), if not found returns None
    

    
    
}