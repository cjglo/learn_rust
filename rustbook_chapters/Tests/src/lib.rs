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

    


}



