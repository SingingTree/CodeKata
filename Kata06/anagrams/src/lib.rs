use std::collections::HashMap;

pub fn get_sorted_chars(string : &str) -> String {
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

#[test]
fn get_anagrams_works_on_simple_data() {
    let strings = ["ate", "eat"];
    let mut expected_anagrams = Vec::new();
    expected_anagrams.push("ate");
    expected_anagrams.push("eat");
    let anagram_map = get_anagrams(&strings);
    assert_eq!(anagram_map.keys().len(), 1);
    assert_eq!(anagram_map["aet"], expected_anagrams);
}
