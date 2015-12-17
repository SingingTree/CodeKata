use std::collections::HashMap;

fn get_sorted_chars(string : &str) -> String {
    let mut chars : Vec<char> = string.chars().collect();
    chars.sort();
    let mut sorted_string = String::new();
    for c in chars {
        sorted_string.push(c);
    }
    sorted_string
}

pub fn get_anagrams<'a>(strings : &[&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut anagram_map = HashMap::new();
    for string in strings {
        let sorted_string = get_sorted_chars(string);
        let string_vec = anagram_map.entry(sorted_string).or_insert(Vec::new());
        string_vec.push(*string);
    }
    anagram_map
}

#[test]
fn get_sorted_chars_works() {
    let string = "waffles";
    let sorted = get_sorted_chars(string);
    assert_eq!(sorted, "aefflsw");
}
