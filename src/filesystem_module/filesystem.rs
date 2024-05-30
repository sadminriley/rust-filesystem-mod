use assert_fs::prelude::*;
use std::io::Write;
use std::fs;
use tempfile::tempdir;


// public module fn
pub fn create_fs(files: &[(String, String)]) -> assert_fs::TempDir {

    // Create tmp directory
    let temp_dir = assert_fs::TempDir::new().expect("Error creating temporary directory!");

    for (name, content) in files {
        let path = temp_dir.path().join(name);
        fs::create_dir_all(path.parent().expect("Invalid parent directory name or location!"))
            .expect("Error creating parent directories!");

    // Create and write file contents
        let mut file = fs::File::create(&path).expect("Error creating file!");
        file.write_all(content.as_bytes())
            .expect("Error! Could not write to file");
    }

    temp_dir // run it!
}