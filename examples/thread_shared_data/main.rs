use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> () {
    println!("main thread started");

    // we create the mutex which allows only a single thread access to the data at any given time
    let counter_mutex = Mutex::new(0); // could nest in Arc::new() but not for this example

    // since this data is being passed to thread, we need to use Arc instead of Rc
    let counter = Arc::new(counter_mutex);

    let mut thread_handles = vec![];

    for _ in 0..5 {
        let counter_arc = Arc::clone(&counter);

        thread_handles.push(thread::spawn(move || {
            // we need to lock it so that no other thread can access it since only one thread
            // should have access to avoid multiple mutation happening at the same time
            let mut count = counter_arc.lock().expect("unable to lock counters");

            *count += 1;
        }));
    }

    for thread_handle in thread_handles {
        thread_handle.join().expect("threads failed to finish");
    }

    println!("{counter:?}",);
    println!("main thread ended");
}
