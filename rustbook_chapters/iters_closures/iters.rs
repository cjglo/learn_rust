// Chapter 13 Iterators

// Just need to impl Item type and next() to impl full Iterator trait

// BIG NOTE: Rust's ownership rules means that many methods completely consume iterators
// ex/ vec has a sum method that sums its elements, this takes ownership and destorys vec

// quick practice with iterators and map


fn main() {

    let v1 = vec![2, 5, 6, 7, 8, 9];

    let v2: Vec<_> = v1.iter().map( |x| { let y = x + 1; return y % 2 == 0; }  ).collect();  // BIG NOTE: MUST CALL COLLECT, 
    // ^^^ iter adapters are lazy and so to collect results you must tell it to return vector

    println!("{:?}", v2);

    // remaing parts of capture, I broke into functions to make it easier to find and write
    println!("");
    closures_capture_env();
    creating_iterator();

}



fn closures_capture_env() {

    let bar_visitors = vec![18, 32, 55, 20, 23, 26, 76, 53, 31];

    let allowed_in: Vec<_>  = bar_visitors.into_iter().filter( |x| *x >= 21 ).collect();

    println!("{:?}", allowed_in);
}


fn creating_iterator() {

    struct Counter {
        count: u32
    }

    impl Counter {

        fn new() -> Counter { 
            Counter {count: 0 } 
        }

    }


    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {

            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }

        }

    }

    // once next is done, all Iterator methods work!
    // long exmaple where we pair two counters (one that skips its first return) then 
    // multiplies the result, then keeps only those divisible by 3 and sums them

    let sum: u32 = Counter::new().zip(Counter::new().skip(1)).map( |(a, b)| a * b ).filter( |x| x % 3 == 0).sum();

    println!("\n{}", sum);
    // Note: Zip only makes 4 pairs because the 2nd iterator returns None and zip throws out that pair
}


