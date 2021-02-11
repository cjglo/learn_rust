// Chapter 12 - Command Line Project

use std::env;
use std::fs;
use std::process;


fn main() {

    println!("\n"); 
    // accepting command line arguements
    let args: Vec<String> = env::args().collect();
    // println!("{:?} \n", args);

    // reading poem text file
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("inside of {}", config.filename);

    run(config);

}

// running configuration and printing result
fn run(config: Config) {

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file!");

    println!("Contents: \n {} \n", contents);
    
}




// configure struct
struct Config {
    query: String,
    filename: String,
}

impl Config {

    // parsing logic for commands
    fn new(args: &[String]) -> Result<Config, &'static str> {

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