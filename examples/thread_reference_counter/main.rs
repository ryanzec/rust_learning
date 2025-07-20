#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() -> () {
    let reference_counter = Rc::new(String::from("rc"));
    let thread_reference_counter = Arc::new(String::from("arc"));

    let handle = thread::spawn(move || {
        println!("{thread_reference_counter}");

        // remember that Rc is not thread safe so when you need that kind of functionality,
        // you need to use Arc
        // println!("{reference_counter}");
    });

    handle.join().expect("thread failed to finish");
}
