//import file system library
use std::fs;
//function to read file info.txt
fn main() {
    let file_contents = fs::read_to_string("info.txt")
        .expect("Should have been able to read the file");
    println!("info.txt context are {file_contents}");
}