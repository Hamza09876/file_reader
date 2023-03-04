// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("info.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        
        println!("1");
        println!("{}", line);
    }

    Ok(())
}