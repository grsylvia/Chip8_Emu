// Import access to filesystem and IO error types
use std::fs;

// Read assembly file as text
let source = fs::read_to_string("test.asm").expect("Failed to read .asm file");

// Look over each line in file and print each on a line newline
for line in source.lines() {
    println!("{line}");
}