use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        match receiver.recv() {
            Ok(data) => {
                // use the data value in the new thread
            }
            Err(err) => {
                // handle the error
            }
        }
    });

    // do some work in the main thread

    let data = 42;
    match sender.send(data) {
        Ok(()) => {}
        Err(_) => {
            // handle error
        }
    };
}
