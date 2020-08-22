use std::env;

// You can get arguments from command line into Rust
// If you type "cargo run {something}" that something is added to the vector below
// since the args.collect() method is there
pub fn run()
{
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Args: {:?}", args);
    println!("command: {}", command);

    // Here is a way to use cargo run hello to run something

    if command == "hello"
    {
        println!("Hello to you too!");
    }
    else
    {
        println!("That is not a valid command!");
    }
}