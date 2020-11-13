

// Mentioned before but .unwrap() allows return of value or throw panic without extra lines
// .expect("something") does the same but makes 'something' the error message for panic




fn read_username_from_file() -> Result<String, io::Error>
{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}



// The ?
// it is a way to simply the 'Result' enum similar to .expect()

// it is essentially the same as

let mut f = File::open("hello.txt");

let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
}

// Notice: IT RETURNS IF ERROR!
// this is same for ? and is way it is there twice.  If either w/ ? get error it returns and passes error up
// otherwise it continues to Ok(s) and returns that