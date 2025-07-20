use std::thread;

fn main() -> () {
    let numbers = vec![1, 2, 3];

    // move tells the closure to take owner of the variables it is using, otherwise the compiler
    // would error about the closure living past the value it is borrowing
    let handle = thread::spawn(move || {
        println!("{numbers:?}");
    });

    // note that because we are moving numbers into the closure of the thread, this code would not compile
    // println!("{numbers:?}");

    handle.join().expect("thread failed to finish");
}
