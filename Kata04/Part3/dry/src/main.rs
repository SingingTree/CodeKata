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

fn parse_table(table : &str, team_name_col : usize, col1 : usize, col2 : usize, trim_characters : &[char]) -> Vec<(String, i32, i32)> {
    let table_lines = table.lines_any();
    let mut parsed_table = Vec::new();
    
    // Skip table header, then ignore any lines that have issues with parsing
    for line in table_lines.skip(1) {
        let words : Vec<&str> = line.split_whitespace().collect();
        let team_name = match words.get(team_name_col) {
            Some(&name) => name,
            None => continue
        };
        let goals_for = match words.get(col1) {
            Some(num) => match num.trim_matches(trim_characters).parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            None => continue
        };
        let goals_against = match words.get(col2) {
            Some(num) => match num.trim_matches(trim_characters).parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            None => continue
        };
        parsed_table.push((team_name.to_owned(), goals_for, goals_against));
    }
    parsed_table
}

fn weather() {
    let table_string = read_file("weather.dat");
    let mut parsed_table = parse_table(&table_string, 0, 1, 2, &['*']);
    // Sort vector by spread -- smallest to greatest
    parsed_table.sort_by(|&(_, a_max, a_min), &(_, b_max, b_min)| (a_max - a_min).cmp(&(b_max - b_min)));
    // Print day with smallest spread
    println!("{}", parsed_table[0].0);
}

fn football() {
    let table_string = read_file("football.dat");
    let mut parsed_table = parse_table(&table_string, 1, 6, 8, &[]);
    // Sort vector by spread -- smallest to greatest
    parsed_table.sort_by(|&(_, a_for, a_against), &(_, b_for, b_against)| ((a_for - a_against).abs()).cmp(&(b_for - b_against).abs()));
    // Print day with smallest spread
    println!("{}", parsed_table[0].0);
}

fn main() {
    weather();
    football();
}