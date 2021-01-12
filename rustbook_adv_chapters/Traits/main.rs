// use std::io;

fn main()
{
	// just practicing traits
	// impl for already defined types:
	trait Describe
	{
		fn describe(&self) -> &str;
	}

	impl Describe for bool
	{
		fn describe(&self) -> &str
		{
		"this is a bool"
		}
	}

	impl Describe for u64
	{
		fn describe(&self) -> &str
		{
		"this is a unsigned 64 bit"
		}
	}

	let myType: bool = true;

	println!("What computer says: {}", myType.describe());


	// Next practice ---------------- :

	trait Client
	{
		// new
		fn new(name: String) -> Self; // note: Capital (S)elf means implementor type

		// membership to stdout
		fn display_mem(&self) -> ();

		// testing points
		fn eligible_for_reward(&self) -> bool;
	}


	struct SilverClient
	{
		name: String,
	}

	struct GoldClient
	{
		name: String,
		points: u8,
	}

	struct PlatClient
	{
		name: String,
		points: u8,
		location: String,
	}

	impl PlatClient
	{
		fn set_location(&mut self, location: String)
		{
			self.location = location;
		}
	}

	impl Client for PlatClient
	{
		// new
		fn new(name: String) -> PlatClient
		{
			PlatClient
			{
				name,
				points: 0,
				location: String::from("undefined"),
			}
		}

		// membership to stdout
		fn display_mem(&self) -> ()
		{
			println!("This is a Platinum Client!");
		}

		// testing points
		fn eligible_for_reward(&self) -> bool
		{
			self.points >= 100
		}

	}


	impl Client for SilverClient
	{
		// new
		fn new(name: String) -> SilverClient
		{
			SilverClient
			{
				name,
			}
		}

		// membership to stdout
		fn display_mem(&self) -> ()
		{
			println!("This is a Silver Client!");
		}

		// testing points
		fn eligible_for_reward(&self) -> bool
		{
			false
		}
		
	}


	
	impl Client for GoldClient
	{
		// new
		fn new(name: String) -> GoldClient
		{
			GoldClient
			{
				name,
				points: 0,
			}
		}

		// membership to stdout
		fn display_mem(&self) -> ()
		{
			println!("This is a Gold Client!");
		}

		// testing points
		fn eligible_for_reward(&self) -> bool
		{
			self.points >= 150
		}
		
	}



}