use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> () {
    let (transmitter, receiver) = mpsc::channel();
    let value_to_send = String::from("value to send to channel");

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        // teh channel take ownership of value_to_send here
        transmitter
            .send(value_to_send)
            .expect("unable to send value to channel");

        // note that data sent to the thread moves ownership so that following code would produce
        // a compile error
        // println!("{value_to_send}");

        println!("thread ended")
    });

    // the channel gives ownership of the value it was send to message here
    let message = receiver
        // recv will wait for a message to be sent to the channel which is why the application does not close right
        // away and instead wait for the thread (that is delayed) to send the message
        .recv()
        .expect("unable to receive message from channel transmitter");

    println!("message from channel: {message}");

    // again since value_to_send has been passed through the change, the ownership of the value
    // has been lost by the value_to_send variable so this would have a compile error
    // println!("{value_to_send}");
}
