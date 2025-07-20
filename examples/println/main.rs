use std::thread;
use std::time::{Duration, Instant};

fn main() -> () {
    let inline_variable = "test";
    let now = Instant::now();

    thread::sleep(Duration::from_millis(10));

    // normal printing
    println!("just a normal string");

    // print the standard version of the variable if available
    println!("{inline_variable}");
    println!("{}", inline_variable);

    // print the debug version of the variable if available
    println!("{inline_variable:?}");
    println!("{:?}", inline_variable);

    // limit value to a certain decimal place
    println!("{:.2?}", Instant::now() - now);
}
