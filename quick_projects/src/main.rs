use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Let me guess your number!");
	
	
	let secret_num = rand::thread_rng().gen_range(1, 101);
	println!("The secret number is {}", secret_num);

	loop 
	{
		println!("Input Number: ");
		let mut guess = String::new();

		io::stdin().read_line(&mut guess).expect("Failed to Read Line");
		
		let guess: u32 = guess.trim().parse()
		{
			// Adding error specs here with ok and err
		}

		println!("you guessed: {}", guess);
		
		match guess.cmp(&secret_num)
		{
			Ordering::Less => println!("Too Small"),
			Ordering::Greater => println!("Too Large"),
			Ordering::Equal => { println!("You Win!"); break; }
		}

	}


}
