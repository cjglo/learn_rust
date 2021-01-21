
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
	


}