use std::sync::mpsc;
use std::thread;

//@todo figure out how to simulate high throughput

fn main() -> () {
    // whenever something is added to a channel, resource are being allocated to that
    // channel and if data is being allocated very fast and the processing of the data
    // can't keep up, that can cause issue. sync_channel allow you so say how many
    // messages can be in the queue before send more data becomes a block process so
    // that your channel does not become too big
    let (transmitter, receiver) = mpsc::sync_channel::<String>(100);

    thread::spawn(move || {
        transmitter
            .send("test".into())
            .expect("unable to send value to channel");

        println!("thread ended")
    });

    let message = receiver
        .recv()
        .expect("unable to receive message from channel transmitter");

    println!("message from channel: {message}");
}
