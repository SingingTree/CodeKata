use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn get_char_freqs(string : &str) -> HashMap<char, usize> {
    let mut freqs = HashMap::new();
    for c in string.chars() {
        let counter = freqs.entry(c).or_insert(0);
        *counter += 1;
    }
    freqs
}

#[test]
fn get_freqs_works() {
    let string = "waffles";
    let freqs = get_char_freqs(string);
    assert_eq!(freqs[&'w'], 1);
    assert_eq!(freqs[&'a'], 1);
    assert_eq!(freqs[&'f'], 2);
    assert_eq!(freqs[&'l'], 1);
    assert_eq!(freqs[&'e'], 1);
    assert_eq!(freqs[&'s'], 1);
}
