// patterns are used quite often in Rust

fn main() {

	// pattern matching is done with "if let" and "while let" but also regular let and for loops

	let x = 5; // technically a pattern between var and value, easier to see on next line
	let (y, z) = (8, 9);

	// functions do pattern matching 
	
	fn foo(&(x,y): &(i32, i32)) {

		// can pass in a tuple
		// like this:
		// let pattern = (23, 53);
		// foo(pattern);	

	}


	




}
