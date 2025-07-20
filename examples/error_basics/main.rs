#![allow(dead_code)]
use std::process;

#[derive(Debug)]
enum MyError {
    Bad,
    NotGood,
}

fn can_error(value: i32) -> Result<i32, MyError> {
    if value > 0 {
        Ok(value)
    } else {
        Err(MyError::Bad)
    }
}

fn main() {
    let result = can_error(5);

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }

    println!("should print");

    let result = can_error(-5);

    if let Err(err) = result {
        println!("Error: {:?}", err);

        process::exit(0);
    }

    println!("should not print");
}
