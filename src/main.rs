//import filesystem_module
mod filesystem_module {
    pub mod filesystem;
}

use filesystem_module::filesystem;
use std::io::Write;
use std::fs;
//use assert_fs::fixture::*;
use assert_fs::prelude::*;

fn main() {
    let files = vec![("file.txt".to_string(), "content".to_string())];
    let temp_dir = filesystem::create_fs(&files);

    temp_dir
        .child("file.txt")
        .assert("content");

    println!("File system created!");

    let file_path = temp_dir.path().join("test.txt");

    // Write content to the file
    let mut file = std::fs::File::create(&file_path).expect("Failed to create file!");

    file.write_all(b"Goodbye, Python!").expect("Failed to write to file!");


    // Check if file exists
    assert!(file_path.exists());
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "Goodbye, Python!");
    
    // Print content to stdout
    let file_contents = fs::read_to_string(&file_path).expect("Failed to read file");
    println!("Loaded File contents: {}", file_contents);
} 