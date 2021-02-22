// Chapter 13:

// Backend of App example 

use std::thread;
use std::time::Duration;

fn main() {

    let simulated_user_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_value,
        simulated_random_number,
    )
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {

    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn slightly_better_generate_workout(intensity: u32, random_number: u32) {

    let expensive_closure = |num: u32| -> u32 { // CLOSURE IS HERE (Don't actually need types defined, but can have)
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };  // We could call similated_expensive_calc in many places, but that makes it difficult to go back and change
    // We could call it first then do brackets, but that wastes time by calling it when not needed
    // closure makes it easy to edit and doesn't make calling unnecessary

    if intensity < 25 {
        println!("Today do {} pushups", expensive_closure(intensity));
        println!("Next do {} situps", expensive_closure(intensity));
    }
    else {
        if random_number < 3 {
            println!("Take a break today");
        }
        else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }

}


// CLosure syntax can be very shrot if needed:
fn syntax_example()
{
    let other_closure = |x| x + 1; // valid syntax for adding one when other_closure() is called

    other_closure(1);
}


// So back to example, we coudl use a Struct to allow us to run function once and then catpure value
struct Cacher<T> 
    where T: Fn(u32) -> u32 // selfr explanatory, but functions are traits!
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {

    fn new(calculation: T) -> Cacher<T>{
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => { 
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// we now can build an ever better version:
fn generate_workout(intensity: u32, random_number: u32) {

    
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today do {} pushups", expensive_result.value(intensity));
        println!("Next do {} situps", expensive_result.value(intensity));
    }
    else {
        if random_number < 3 {
            println!("Take a break today");
        }
        else {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }

}


// More on CLosures:
// another thing closures can do is capture their envirnment, aka use variables in their envirnment that are not parameters

fn capture_example() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    equal_to_x(y); // if you did assert!() this would pass, x is saved despite not being a parameter

    // Also note: Fn traits have three options
    // FnOnce -> take ownership
    // FnMut -> mut & 
    // Fn -> const &


    // could use FnOnce by using "move" in closure to take ownership
    let equal_to_y = move |num| num == y; // y is taken because move keyword in defintion

    // y can no longer be used here
}


