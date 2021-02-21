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

fn generate_workout(intensity: u32, random_number: u32) {

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



