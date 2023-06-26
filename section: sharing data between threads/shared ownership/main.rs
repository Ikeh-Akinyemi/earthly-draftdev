use std::sync::Arc;
use std::thread;

fn main() {
    let shared_data = Arc::new(42);

    let handle = thread::spawn({
        let shared_data = shared_data.clone();

        move || {
            // use the shared_data value in the new thread
        }
    });

    // do some work in the main thread...

    let result = match handle.join() {
        Ok(result) => {
            // handle success case with result
        },
        Err(_) => {
            // handle error case
        }
    };
}
