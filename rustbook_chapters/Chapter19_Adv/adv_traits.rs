
use std::ops::Add;


fn main() {

	// there are a lot of adv feature sfor traits, generic parameters, ect around pg ~420 in the book



	// I will write out an overloading operator with a trait and using generic paramter types


	struct Millimeters(u32);
	struct Meters(u32);

	impl Add<Meters> for Millimeters {
		type Output = Millimeters;

		fn add(self, other: Meters) -> Millimeters {
		
			Millimeters(self.0 + (other.0 * 1000))
				
		}

	}
		

	// Add trait reads as "impl Add<RHS=Self>  where RHS becomes second param type in add()
	// If left blank RHS is Self, but in the above we wanted to specify meters

}


