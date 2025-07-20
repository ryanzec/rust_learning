use std::sync::mpsc;
use std::thread;

fn main() -> () {
    println!("main thread started");

    let (transmitter, receiver) = mpsc::channel::<String>();

    thread::spawn(move || {
        transmitter
            .send("test1".into())
            .expect("unable to send value to channel");

        transmitter
            .send("test2".into())
            .expect("unable to send value to channel");

        println!("thread ended")
    });

    // handling multiple messages to a receive is just a matter of looping through them like
    // any iterator
    for received_message in receiver {
        println!("message from channel: {received_message}");
    }

    println!("main thread ended");
}
