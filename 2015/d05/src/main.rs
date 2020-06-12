use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const EXCLUDES: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn contains_vowels(string: &str, qtity: usize) -> bool {
    let mut i = 0;
    for c in string.chars() {
	if VOWELS.contains(&c) {
	    i += 1;
	}
    }
    if i >= qtity {
	return true;
    }
    false
}

fn has_char_repetition(string: &str) -> bool {
    let mut last_char = 'a';
    for (i, c) in string.chars().enumerate() {
	if c == last_char && i > 0 {
	    return true;
	}
	last_char = c;
    }
    false
}

fn contains_one_substr(string: &str, substrs: &[&str]) -> bool {
    for substr in substrs.iter() {
	if string.contains(substr) {
	    return true;
	}
    }
    false
}

fn string_is_nice1(string: &str) -> bool {
    if contains_vowels(string, 3) {
	if has_char_repetition(string) {
	    if ! contains_one_substr(string, &EXCLUDES) {
		return true;
	    }
	}
    }
    false
}


fn has_repeating_chars_pair_without_overlapping(string: &str) -> bool {
    // take letter 1 & 2, see if there is a repetition at pos 3 & 4, 4 & 5 ...
    // if not, check letter 2 & 3...
    // /!\ excepts ascii
    let chars = string.as_bytes();
    if chars.len() <= 3 {
	return false;
    }
    let mut char_a = 0;
    let mut char_b = 1;
    for i in 2..chars.len() {
	for j in i..chars.len() - 1 {
	    if &chars[char_a] == &chars[j] && &chars[char_b] == &chars[j + 1] {
		return true;
	    }		
	}
	char_a += 1;
	char_b += 1;
    }
    false
}

// at least one letter which repeats with exactly one letter between them
fn has_repeating_chars_with_one_between(string: &str) -> bool {
    // /!\ excepts ascii    
    let chars = string.as_bytes();
    // enumerate always starts at 0
    // we're starting at idx 2
    // and looking for a similar letter 2 chars before
    // so we can just i directly as a -2 offset
    for (i, c) in chars[2..].iter().enumerate() {
	if c == &chars[i] {
	    return true;
	}
    }
    false
}

fn string_is_nice2(string: &str) -> bool {
    if has_repeating_chars_pair_without_overlapping(string) {
	if has_repeating_chars_with_one_between(string) {
	    return true;
	}
    }
	false
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_has_repeating_chars_pair_without_overlapping() {
	assert!(has_repeating_chars_pair_without_overlapping("xyxy"));
	assert!(has_repeating_chars_pair_without_overlapping("aabcdefgaa"));
	assert!(! has_repeating_chars_pair_without_overlapping("aaa"));
	assert!(has_repeating_chars_pair_without_overlapping("aaaa"));
	assert!(has_repeating_chars_pair_without_overlapping("abfhfiaohfowabakop"));
	assert!(! has_repeating_chars_pair_without_overlapping("baaab"));
    }

    #[test]
    fn test_has_repeating_chars_with_one_between() {
	assert!(has_repeating_chars_with_one_between("abcdefeghi"));
	assert!(has_repeating_chars_with_one_between("xyx"));
	assert!(has_repeating_chars_with_one_between("aaa"));	
	assert!(! has_repeating_chars_with_one_between("abc"));
    }

    #[test]
    fn test_string_is_nice2() {
	assert!(string_is_nice2("qjhvhtzxzqqjkmpb"));
	assert!(string_is_nice2("xxyxx"));
	assert!(! string_is_nice2("uurcxstgmygtbstg"));
	assert!(! string_is_nice2("ieodomkazucvgmuy"));
    }
}


fn main() {
    let file = File::open("data.txt")
	.expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut i = 0;
    let mut j = 0;
    for line in buf_reader.lines() {
	let line_ = line.unwrap();
	if string_is_nice1(&line_) {
	    i += 1;
	}
	if string_is_nice2(&line_) {
	    j += 1;
	}
    }
    println!("{}", i);
    println!("{}", j);
 }
