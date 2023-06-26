use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn count_to_10(shared_data: Arc<(Mutex<u32>, Condvar)>, thread_num: u32) {
    let &(ref mutex, ref cvar) = &*shared_data;
    let mut count = mutex.lock().unwrap();

    while *count < 10 {
        if *count % 3 == thread_num {
            *count += 1;
            cvar.notify_all();
        } else {
            count = cvar.wait(count).unwrap();
        }
    }
}

fn main(){}

#[test]
fn test_count_to_10() {
    let shared_data = Arc::new((Mutex::new(0), Condvar::new()));
    let mut handles = Vec::new();

    for i in 0..3 {
        let shared_data = shared_data.clone();
        let handle = thread::spawn(move || {
            count_to_10(shared_data, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}