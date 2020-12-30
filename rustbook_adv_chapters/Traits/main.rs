
fn main()
{
	// just practicing traits

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



}