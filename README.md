# rust-filesystem-mod
Some ideas to get the Rust gears turning for Temporary file systems in code


## Run
```
$ cargo run
File system created!
Loaded File contents: Goodbye, Python!
```

## Ideas for functionality
- Write to file
- Create a temporary file/directory
- Created nested directories
- use drop() to delete all files used in tmpfs


Drop could be like this in code:
```
drop(temp_dir);


assert!(!temp_dir.path().exists());
```
