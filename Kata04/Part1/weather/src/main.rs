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

fn parse_table(table : &str) -> Vec<(i32, i32, i32)> {
    let table_lines = table.lines_any();
    let mut parsed_table = Vec::new();
    
    // Skip table header, then ignore any lines that have issues with parsing
    for line in table_lines.skip(2) {
        let mut words = line.split_whitespace();
        let day = match words.next() {
            Some(num) => match num.parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            None => continue
        };
        let max_temp = match words.next() {
            Some(num) => match num.trim_matches('*').parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            None => continue
        };
        let min_temp = match words.next() {
            Some(num) => match num.trim_matches('*').parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            None => continue
        };
        parsed_table.push((day, max_temp, min_temp));
    }
    parsed_table
}

fn main() {
    let table_string = read_file("weather.dat");
    let mut parsed_table = parse_table(&table_string);
    // Sort vector by spread -- smallest to greatest
    parsed_table.sort_by(|&(_, a_max, a_min), &(_, b_max, b_min)| (a_max - a_min).cmp(&(b_max - b_min)));
    // Print day with smallest spread
    print!("{}", parsed_table[0].0);
}
