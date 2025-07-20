use rayon::prelude::*;
use std::time::Instant;

// BigUnit needs to support values higher than 35
use num::{BigUint, One};

// un-threaded version
fn factorial(number: u32) -> BigUint {
    if number == 0 || number == 1 {
        return BigUint::one();
    }

    return (1..=number)
        .map(BigUint::from)
        .reduce(|collector, index| collector * index)
        .expect("could not reduce numbers");
}

// threaded version
fn factorial_threaded(number: u32) -> BigUint {
    if number == 0 || number == 1 {
        return BigUint::one();
    }

    return (1..=number)
        // this creates an iterator that is parallelized by rayon
        .into_par_iter()
        .map(BigUint::from)
        .reduce(|| BigUint::one(), |collector, index| collector * index);
}

fn main() -> () {
    let now = Instant::now();

    // uncomment this to see how much slower it is
    factorial(50000);

    println!("non-threaded: {:?}", Instant::now() - now);

    let now = Instant::now();

    factorial_threaded(50000);

    println!("threaded: {:?}", Instant::now() - now);
}
