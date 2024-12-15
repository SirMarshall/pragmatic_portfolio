use std::io;
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_input() -> String {
   let mut input = String::new();
   io::stdin()
      .read_line(&mut input)
      .expect("Failed to read input. Provide more input next time.");
   input.trim().to_string() // This should fix it.
}

pub fn get_dir() -> PathBuf {
   current_dir().expect("Failed to get current directory.")
}