// Chapter 12 - Command Line Project
use std::env;
use std::process;
use minigrep::Config;


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

    
    

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }

}

