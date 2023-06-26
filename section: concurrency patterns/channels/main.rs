use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn worker(receiver: mpsc::Receiver<Instant>) {
    loop {
        let deadline = match receiver.recv() {
            Ok(deadline) => deadline,
            Err(_) => break,
        };

        let now = Instant::now();
        if now >= deadline {
            println!("Worker received a task after the deadline!");
        } else {
            let remaining_time = deadline - now;
            println!("Worker received a task. Deadline in {:?}.", remaining_time);
            thread::sleep(remaining_time);
            println!("Task completed!");
        }
    }
}

fn main() {
  let (sender, receiver) = mpsc::channel();

  let worker_handle = thread::spawn(move || {
      worker(receiver);
  });

  // Sending tasks with different deadlines
  sender.send(Instant::now() + Duration::from_secs(3)).unwrap();
  sender.send(Instant::now() + Duration::from_secs(5)).unwrap();
  sender.send(Instant::now() + Duration::from_secs(7)).unwrap();
  sender.send(Instant::now() + Duration::from_secs(4)).unwrap();

  // Signal no more tasks and wait for the worker to finish
  drop(sender);
  worker_handle.join().unwrap();
}
