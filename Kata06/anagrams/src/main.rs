extern crate anagrams;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use anagrams::get_anagrams;
use anagrams::get_sorted_chars;

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
    let words = read_file("wordlist.txt");
    let words_lines : Vec<&str> = words.lines().collect();
    let anagrams = get_anagrams(&words_lines);
    for anagram in anagrams[&get_sorted_chars("crepitus")].iter() {
        print!("{} ", anagram);
    }
    println!("");
}
