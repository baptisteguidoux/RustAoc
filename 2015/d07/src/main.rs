use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;


fn get_value<'a>(wire: &'a str, signals: &'a HashMap<String, String>, cache: Rc<RefCell<HashMap<&'a str, u16>>>) -> u16 {

    println!("looking for key: [{}]", wire);

    if let Ok(val) = wire.parse::<u16>() {
	println!("Found value: [{}]", val);
	return val;
    }

    if let Some(val) = cache.borrow().get(wire) {
	return *val;
    }

    match signals.get(wire) {
	Some(operation) => {
	    let split_op: Vec<&str> = operation.split_whitespace().into_iter().collect();

	    if split_op.contains(&"AND") {
		let res = get_value(split_op[0], signals, Rc::clone(&cache)) & get_value(split_op[2], signals, Rc::clone(&cache));
		cache.borrow_mut().insert(wire, res);
		return res;
	    } else if split_op.contains(&"OR") {
		let res = get_value(split_op[0], signals, Rc::clone(&cache)) | get_value(split_op[2], signals, Rc::clone(&cache));
		cache.borrow_mut().insert(wire, res);
		return res;
	    } else if split_op.contains(&"LSHIFT") {
		let res = get_value(split_op[0], signals, Rc::clone(&cache)) << get_value(split_op[2], signals, Rc::clone(&cache));
		cache.borrow_mut().insert(wire, res);
		return res;
	    } else if split_op.contains(&"RSHIFT") {
		let res = get_value(split_op[0], signals, Rc::clone(&cache)) >> get_value(split_op[2], signals, Rc::clone(&cache));
		cache.borrow_mut().insert(wire, res);
		return res;
	    } else if split_op.contains(&"NOT") {
		let res = !get_value(split_op[1], signals, Rc::clone(&cache));
		cache.borrow_mut().insert(wire, res);
		return res;
	    } else {
		let res = get_value(split_op[0], signals, Rc::clone(&cache));
		cache.borrow_mut().insert(wire, res);
		return res;
	    }
	},
	None => {
	    panic!("key {} not found");
	}
    }
}

fn main () {

    let file = File::open("data.txt").expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut signals: HashMap<String, String> = HashMap::new();
    let cache: Rc<RefCell<HashMap<&str, u16>>> = Rc::new(RefCell::new(HashMap::new()));

    let test_input = vec![
	"123 -> x",
	"456 -> y",
	"x AND y -> d",
	"x OR y -> e",
	"x LSHIFT 2 -> f",
	"y RSHIFT 2 -> g",
	"NOT x -> h",
	"NOT y -> i"
    ];

    for line in buf_reader.lines() {
	let instruction = line.unwrap();
	let mut split_instr = instruction.split(" -> ");
	let operation = split_instr.next().unwrap();
	let wire = split_instr.next().unwrap();
	signals.insert(wire.to_string(), operation.to_string());
    }

    let wire_a_val = get_value("a", &signals, Rc::clone(&cache));

    println!("{}", wire_a_val);

    cache.borrow_mut().clear();

    cache.borrow_mut().insert("b", wire_a_val);

    let wire_a_val = get_value("a", &signals, Rc::clone(&cache));

    println!("{}", wire_a_val);
}
