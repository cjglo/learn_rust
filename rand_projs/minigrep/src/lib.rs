use std::fs;
use std::error::Error;

// running configuration and printing result
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search_case_insensitive<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {

    vec![]
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut vec = vec![];
    
    for line in contents.lines() {
        if line.contains(query) {
            vec.push(line);
        }
    }
    
    vec
}


// configure struct
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {

    // parsing logic for commands
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(
            Config {
            query,
            filename,
            }
        )

    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}