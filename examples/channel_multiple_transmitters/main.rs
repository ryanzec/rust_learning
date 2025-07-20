use std::sync::mpsc;
use std::thread;

fn main() -> () {
    let (transmitter, receiver) = mpsc::channel::<String>();

    // you might need to have multiple transmitters for a single channel (like if you need to
    // use a transmitter in multiple threads) so in those cases, you can just to .clone() or
    // the original transmitter
    let transmitter2 = transmitter.clone();

    thread::spawn(move || {
        println!("thread1 started");

        // this makes it so these messages are seen the second thread messages
        thread::sleep(std::time::Duration::from_millis(500));

        transmitter
            .send("test1".into())
            .expect("unable to send value to channel");

        thread::sleep(std::time::Duration::from_millis(100));

        transmitter
            .send("test2".into())
            .expect("unable to send value to channel");

        thread::sleep(std::time::Duration::from_millis(100));

        println!("thread1 ended");
    });

    thread::spawn(move || {
        println!("thread2 started");

        thread::sleep(std::time::Duration::from_millis(100));

        transmitter2
            .send("test1 (cloned transmitter)".into())
            .expect("unable to send value to channel");

        thread::sleep(std::time::Duration::from_millis(100));

        transmitter2
            .send("test2 (cloned transmitter)".into())
            .expect("unable to send value to channel");

        thread::sleep(std::time::Duration::from_millis(100));

        println!("thread2 ended");
    });

    // handling multiple messages to a receive is just a matter of looping through them like
    // any iterator
    for received_message in receiver {
        println!("message from channel: {received_message}");
    }
}
