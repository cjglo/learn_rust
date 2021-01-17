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

	let cl1 = SilverClient::new(String::from("John"));
	let cl2 = GoldClient::new(String::from("Sam"));
	let cl3 = PlatClient::new(String::from("Beth"));


	cl1.display_mem();
	// println!("It is {} that this member has enough points", cl1.eligible_for_reward());

	cl2.display_mem();
	// println!("It is {} that this member has enough points", cl2.eligible_for_reward());

	cl3.display_mem();
	// println!("It is {} that this member has enough points", cl3.eligible_for_reward());


	// Traits as parameters

	pub fn notify(person: impl Client) // short-hand
	{
		println!("It is {} that this member has enough points", person.eligible_for_reward());
	}

	notify(cl2);
	notify(cl3);

	// if we wanted to force many parameters to be the same type, must use the long-hand:
	pub fn compareSameType<T: Client>(person1: T, person2: T)
	{
		println!("Just made this for example...");
	}

	// Can require multiple traits like this:
	pub fn anotherExample(item: impl Client + Clone) // or anotherExample<T: Client + Describe>( etc. )
	{
		println!("Also example...");
	}

	// For many trait bounds, there is a useful short-hand:
	// The WHERE syntax

	pub fn some_func<T, U>(t: T, u: U) -> ()
		where T: Describe,
			  U: Clone + Client,
	{
		println!("Still does nothign");
	}


	// practice from page 190

	fn largest<T: PartialOrd + Copy>(list: &[T]) -> T
	{
		let mut largest = list[0];
		for &item in list.iter()
		{
			if largest < item
			{
				largest = item;
			}
		}

		largest
	}

	let example = [2, 3, 4, 9, 5];

	println!("{}", largest(&example));
	println!("----");

	


}