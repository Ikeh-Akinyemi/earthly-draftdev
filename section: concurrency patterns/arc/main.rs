use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, BufRead};
use std::thread;

fn main() {
  // File paths to read
  let file_paths = vec!["file1.txt", "file2.txt", "file3.txt"];
  let shared_data = Arc::new(Mutex::new(Vec::new()));

  let mut handles = vec![];

  // Spawn threads to read files concurrently
  for file_path in file_paths {
      let shared_data = Arc::clone(&shared_data);
      let handle = thread::spawn(move || {
          // Open the file
          let file = File::open(file_path).expect("Failed to open file");

          // Read the contents of the file
          let lines: Vec<String> = io::BufReader::new(file)
              .lines()
              .map(|line| line.expect("Failed to read line"))
              .collect();

          // Lock the shared data
          let mut data = shared_data.lock().unwrap();

          // Append the lines to the shared data
          data.extend(lines);
      });

      handles.push(handle);
  }

  // Wait for all threads to finish
  for handle in handles {
      handle.join().unwrap();
  }

  print!("{:#?}", shared_data.lock().unwrap());

  // Explicitly drop the Arc
  drop(shared_data);

  // Attempting to access the shared data after dropping the Arc would result in a compile-time error
  // Uncomment the following line to see the error:
  // let data = shared_data.lock().unwrap();
}

