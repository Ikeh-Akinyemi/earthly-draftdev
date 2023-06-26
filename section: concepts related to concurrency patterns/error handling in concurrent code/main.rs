use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();
    let handle = thread::spawn(move || {
        // Perform som
        // Send the result over the channel
        let send_result = tx.send(result);
        match send_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    });
    // Wait for the thread to finish and handle any errors that occur
    let thread_result = handle.join().unwrap(); // Note: unwrap is safe here because we're propagating any errors through the Result type
    match thread_result {
        Ok(_) => {
            // Receive the result from the channel
            let result = rx.recv();
            match result {
                Ok(val) => println!("Result: {}", val),
                Err(e) => println!("Error receiving result: {:?}", e),
            }
        }
        Err(e) => println!("Error sending result: {:?}", e),
    }
}
