use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // code to be executed in the new thread
    });
    
    match handle.join() {
        Ok(result) => {
            // handle success case with result
        },
        Err(_) => {
            // handle error case
        }
    };
}
