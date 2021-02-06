// *** Automated Testing, Chapter 11 ***

// run all tests with cargo test

#[cfg(test)]
mod tests {

    #[test] // indicates function as test, will be run with `cargo test`
    fn adding_func() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("This test will fail"); // if uncommented
    }

    // assert!() macro does nothing if value evalutes to true, and calls panic!() when it does false
    // this allows us to build out our tests

    fn is_bigger(x: i32, y: i32) -> bool // returns bool so works well for assert
    {
        x > y
    }

    #[test]
    fn testing_is_bigger()
    {
        assert!(is_bigger(14 , 3));
    }

    // ** assert_eq and assert_ne
    // pretty straight forward, creates the bool for us
    fn adds_two(x: i32) -> i32 {
        x + 2
    }

    #[test]
    fn tests_add_two() {
        assert_eq!(3, adds_two(1));
    }

    #[test]
    fn tests_add_two_again() {
        assert_ne!(6, adds_two(1));
    }

    // will mention "(left == right)" in error message if above failed, signalling 
    // what condtion in the above caused the panic (allowing to see why test panicked)

    // called left and right so that can change the order and it won't matter

    // Note: Must impl PartialEq and Debug Traits for types to use assert_eq and _ne


    

    // can also write custom fail messages, like printing the result after a fail    
	#[test]
	#[ignore] // placed ignore so this will be ignored and tests directory is tested (uncomment for this ex)
	fn greeting() {
		let result = "Hello, John!";
		assert!(result.contains("Matt"), 
			"Greeting didn't contain Matt, it was: `{}`", result);

	}



	// may want to make sure that your program panics when a certain value
	// is generated or used.  THis is where `should_panic` comes in

	struct Guess {
	val: i32,
	}

	impl Guess {
	
		fn new(input: i32) -> Guess {
			if input > 100 {
				panic!("guesses should be less than 100");
			}

			Guess {
			 val: input,
			}
		}
	}



	#[test]
	#[should_panic]
	fn guess_out_of_range() {
		Guess::new(102);
	}

	// can add a way to make sure the exact error we wanted is what panics:

	#[test]
	#[should_panic(expected = "guesses should be less than")] // since this is a substring of panic! message, this passes
	fn guess_out_of_range_2() {
		Guess::new(103); // if a different panic triggered w/ diff error message, this would still fail
	}



	// we can also write tests using Result<T,E>
	#[test]
	fn test_with_result() -> Result<(), String> {
	
		if 2 + 2 == 4 { 
			Ok(())
		}
		else {
			Err(String::from("math is broken"))
		}
	
	}	
	// NOTE: Don't use should_panic, have error type returned directly
	// test will panic if Err is returned
	// this allows easy way to write by using ? operator



	// cargo test has many flags that can change how it operates
	// some ones worth noting:
		// runs simultaneous by default, can run one at a time
		// all std out on passes is captured and not printed, can make them printed (like failed tests do by default)
		// carog test --help can give assiatnce
		// can pass names of tests to get specific ones to run alone
		// ^ looks at substrings, so can run multiple by using a piece in name of many tests
	


	// ignore tag lets a test be skipped
	#[test]
	#[ignore]
	fn ignored_test() { // will not be tested and console will log a test as ignored
		assert!(true);
	}
	// can run cargo test -- --ignored, to run ignored tests
	


}



