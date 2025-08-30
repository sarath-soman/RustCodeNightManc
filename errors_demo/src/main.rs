use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), io::Error> {
    match read_file("Cargo.toml") {
        Ok(txt) => println!("Read {} bytes", txt.len()),
        Err(e) => eprintln!("Error: {e}"),
    }
    Ok(())
}