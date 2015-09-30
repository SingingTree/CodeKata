use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_file(file_name : &str) -> String {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
            Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
            Error::description(&why)),
        Ok(_) => s,
    }
}

fn main() {
    read_file("weather.dat");
}
