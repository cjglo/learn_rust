use std::io;

fn main() {
    println!("Let me guess your number!");
    println!("Input Number: ");

   	let mut guess = String::new();

   	io::stdin().read_line(&mut guess).expect("Failed to Read Line");

   	println!("you guessed: {}", guess);

}
