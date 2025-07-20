use std::thread;
use std::time::Duration;

fn main() -> () {
    println!("main thread started");

    let handle = thread::spawn(|| {
        println!("starting thread");

        thread::sleep(Duration::from_secs(1));

        println!("exiting thread");
    });

    thread::sleep(Duration::from_millis(250));
    println!("main thread slept for 250ms");
    thread::sleep(Duration::from_millis(250));
    println!("main thread slept for 250ms");
    thread::sleep(Duration::from_millis(250));
    println!("main thread slept for 250ms");
    thread::sleep(Duration::from_millis(250));
    println!("main thread slept for 250ms");
    thread::sleep(Duration::from_millis(250));
    println!("main thread slept for 250ms");

    // .join() stops the execute of the main thread until this thread related to the handle
    // completes
    handle.join().expect("thread failed to finish");

    println!("main thread finished");
}
