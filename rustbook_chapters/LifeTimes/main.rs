use std::fmt::Display;

fn main()
{
	// quick example as a function that returns longest string slices
	// '{lowercase short name} is the syntax for lifetimes
	
	// &i32; // int ref
	// &'a i32; // int ref w/ lifetime
	// &'a mut i32; // mut int ref w/ lifetime


	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str // remember, not changing anything, telling the compiler what needs to be the case
	{
		// we are saying: "this takes two parameters that will live as long as 'a
		// and we will return something that will have to live as long as 'a
		if x.len() > y.len()
		{
			x
		}
		else
		{
			y
		}
	}

	// structs with references
	struct ImportantExcerpt<'a> // compiler will now check if reference to part lasts as long as struct does
	{
		part: &'a str,
	}


	// aka when print this, need to have the lifetimes last at least as long as result
	let string1 = "long string";
	let string2 = "short string";

	{
		let result = longest(string1, string2);
		println!("Doing other stuff before result is out of scope!");
		// result will go out of scope here
	}

	// works because the lifetimes of the parameters is at least as long as result
	// if `let result;' was put above and string1 was in the block, it wouldn't work


	// KEY NOTE: When you return a reference, you must always specify a life-time! This is
	// because either it refers to one of the paramters (so needs to last that long) or
	// it refers to something in the function (which would go out of scope)
	// aka BY DEFINTION the lifetimes must be specified, thats what returning a reference is
	// Short-hand DOES EXIST that lets you skip writing out 'a, but its because the Rust
	// compiler is including it, not because it isn't necessary


	// compiler follows 3 rules to try and create lifetimes: every paramters gets its own lifetime,
	// if only 1 paramter then everything gets that lifetime, and finally if their arw mulitple lifetimes
	// and one of them is &self, then everything gets self's lifetime



	// lifetimes with methods
	impl<'a> ImportantExcerpt<'a> {
		fn announce_and_return(&self, announcement: &str) -> &str {
			println!("Attention: {}", announcement);
			self.part
		}
	}
	// notice: third rule means self lifetime is applied to the return, so no specification
	// is needed


	// note: 'static is used to designate that something lives the entire program's lfietime

	// also, lifetimes and traits are both generic, so listed toegther
	fn new_longest<'a, T>(x: &'a str, y: T) -> &'a str
		where T: Display
	{
		println!("{}", y);
		return x
	}


}