use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::TryFrom;


fn main() {
    let file = File::open("data.txt").expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut res_a: isize = 0;
    let mut res_b: isize = 0;
    for line in buf_reader.lines() {
	let line = line.unwrap();
	let mut cnt_a = 0;
	let mut cnt_b = 0;
	let mut in_escape_char = false;
	for (i, chr) in line.chars().enumerate() {
	    if in_escape_char {
		if chr == '\\' || chr == '\"' {
		    cnt_a += 1;
		    cnt_b += 2;
		    in_escape_char = false;
		} else if chr == 'x' {
		    // \x27 is one char
		    cnt_a -= 1;
		    cnt_b += 1;
		    in_escape_char = false;
		} else {
		    panic!("WTF?");
		}
	    } else if (i == 0 && chr == '\"')
	      || (i == line.len() - 1 && chr == '\"') {
		  //start or end, do not count a
		  cnt_b += 2;
	    } else if chr == '\\' {
		// do not count yet
	          in_escape_char = true;
		  cnt_b += 2;
	      } else {
		if chr == '\"' {
		    cnt_b += 2;
		}
		  cnt_a += 1;
		  cnt_b += 1;
	    }
	}
	cnt_b += 2; // start and end parentheses
	match isize::try_from(line.len()) {
	    Ok(len) => {
		res_a += len - cnt_a;
		res_b += cnt_b - len;
	    },
	    Err(err) => {panic!("BLA");}
	}
    }
    println!("{}, {}", res_a, res_b);
}
