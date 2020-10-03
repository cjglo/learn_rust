

fn main()
{
    let word = String::from("Ferris Rocks");

    fn first_word(s: &String) -> &str 
    {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate()
        {
            if item == b' ' 
            {
                return &s[..i];
            }
        }

        &s[..]
    }

    let to_print = first_word(&word);

    println!("{:?}", to_print);

}