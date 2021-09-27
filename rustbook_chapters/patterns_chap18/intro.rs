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


	// destruct a struct
	struct Point { x: i32, y: i32 };

	let pt = Point { x: 0, y: 7 };

	let Point {x,y} = pt;

	// println!("Here is x {0}, and here is y {1}", x, y);


	
	// destructing enums is the same, but how the data is defined
	// must be the same in branches as it is in enum defintion

	// aka struct-like enum options need {} w/ variables and tuple-like 
	// need (), and those with none need none

	enum Message {
		Quit,
		Move {x: i32, y: i32},
		ChangeColor(i32, i32 ,i32)
	}

	let msg = Message::Move{x: 23, y: 25};

	match msg {
		Message::Quit => println!("Quit now"),
		Message::Move {x, y} => {
			// println!("move to {0}, {1}", x, y);
		},
		Message::ChangeColor(r,b,g) => {
			// println!("The color is now {0},{1},{2}", r, b, g);
		},
	}

	// ^^^ notice that they each macth exactly how data is defined in enum


	// Can also do this nested

	enum Color {
		Rgb(i32, i32, i32),
		Hsv(i32, i32, i32)
	}

	enum Event {
		Quit,
		Write(String),
		ChangeColor(Color)
	}


	let event = Event::ChangeColor(Color::Hsv(0, 160, 255));

	match event {
		Event::ChangeColor(Color::Rgb(r,g,b)) => {
			// println!("RED GREEN BLUE! {0},{1},{2}", r, g, b);
		},
		Event::ChangeColor(Color::Hsv(h, s, v)) => {
			// println!("HSV! {0},{1},{2}", h, s, v);
		},
		_ => ()
	}


	// Helpful example of using patterns to set variables

	let mut setting_value = None;
	let mut new_setting_value = Some(10);

	match (setting_value, new_setting_value) {
		(Some(_), Some(_)) => {
			println!("Can't overwrite an existing value")
		},
		_ => {
			setting_value = new_setting_value;
		}
	}

	println!("new value: {:?}", setting_value);


	// worth noting that putting an underscore in front of variable will move variable
	// but calling just _ will not

	// aka a if let with Some(_s) will move the variable while Some(_) will not

	
}
