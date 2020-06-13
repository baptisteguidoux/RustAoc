use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


#[derive(Debug, PartialEq)]
enum InstructionAction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    action: InstructionAction,
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
}

impl Instruction {
    fn new(string: &str) -> Instruction {
	let mut action = InstructionAction::TurnOn;

	if string.starts_with("turn on") {
	    // nothing to do
	} else if string.starts_with("turn off") {
	    action = InstructionAction::TurnOff;
	} else if string.starts_with("toggle") {
	    action = InstructionAction::Toggle;
	} else {
	    panic!("string not matching patterns");
	}
	let coordinates = parse_coordinates(string);
	let x_start = coordinates.0;
	let y_start = coordinates.1;
	let x_end = coordinates.2;
	let y_end = coordinates.3;

	Instruction{action, x_start, y_start, x_end, y_end}
    }
}


fn parse_coordinates(string: &str) -> (usize, usize, usize, usize) {
    let coordinates_re = Regex::new(r"(\d{1,3}),(\d{1,3})\sthrough\s(\d{1,3}),(\d{1,3})").unwrap();
    for captured in coordinates_re.captures_iter(string).take(1) {
	return (captured[1].parse().unwrap(), captured[2].parse().unwrap(),
		captured[3].parse().unwrap(), captured[4].parse().unwrap());
    }
    (0, 0, 0, 0)
}

fn apply_instruction_to_lights(instruction: &Instruction, lights: &mut Vec<Vec<bool>>) {
	    for x in instruction.x_start..=instruction.x_end {
		for y in instruction.y_start..=instruction.y_end {
		    match instruction.action {
			InstructionAction::TurnOn => lights[x][y] = true,
			InstructionAction::TurnOff => lights[x][y] = false,
			InstructionAction::Toggle =>  lights[x][y] = !lights[x][y]
		    }
		}
	    }
}

fn apply_instruction_to_lights2(instruction: &Instruction, lights: &mut Vec<Vec<usize>>) {
	    for x in instruction.x_start..=instruction.x_end {
		for y in instruction.y_start..=instruction.y_end {
		    match instruction.action {
			InstructionAction::TurnOn => lights[x][y] += 1,
			InstructionAction::TurnOff => {
			    if lights[x][y] == 0 {
				// nothing
			    } else {
				lights[x][y] -= 1;
			    }
			}
			InstructionAction::Toggle =>  lights[x][y] += 2
		    }
		}
	    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_coordinates() {
	assert_eq!(parse_coordinates("turn on 0,0 through 999,999"), (0, 0, 999, 999));
	assert_eq!(parse_coordinates("toggle 0,0 through 999,0"), (0, 0, 999, 0));
	assert_eq!(parse_coordinates("turn off 499,499 through 500,500"), (499, 499, 500, 500));
    }

    #[test]
    fn test_new_instruction_from_string() {
	let instruction1 = Instruction {
	    action: InstructionAction::TurnOn,
	    x_start: 0,
	    y_start: 0,
	    x_end: 999,
	    y_end: 999,
	};
	assert_eq!(Instruction::new("turn on 0,0 through 999,999"), instruction1);
    }

    #[test]
    fn test_apply_instruction_to_lights() {
	let mut lights = vec![vec![false; 1000]; 1000];
	let instruction = Instruction::new("turn on 0,0 through 999,999");
	apply_instruction_to_lights(instruction, &mut lights);
	assert_eq!(lights, vec![vec![true; 1000]; 1000]);

	let mut lights = vec![vec![false; 1000]; 1000];
	let instruction = Instruction::new("toggle 0,0 through 999,0");
	apply_instruction_to_lights(instruction, &mut lights);
	let mut expected_res = vec![vec![false; 1000]; 1000];
	for x in 0..1000 {
	    expected_res[x][0] = true;
	}
	assert_eq!(lights, expected_res);
    }
}

fn main() {
    let file = File::open("data.txt").expect("could not open file");
    let buf_reader = BufReader::new(file);
    
    // We start with a 1000x1000 grid with lights turned off
    let mut lights = vec![vec![false; 1000]; 1000];
    let mut lights2: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for line in buf_reader.lines() {
	let instruction = Instruction::new(&line.unwrap());
	apply_instruction_to_lights(&instruction, &mut lights);
	apply_instruction_to_lights2(&instruction, &mut lights2);
    }

    let mut lit_lights = 0;
    for row in lights {
	lit_lights += row.iter().filter(|x| *x == &true).count();
    }
    println!("{:?}", lit_lights);

    let mut total_brightness = 0;
    for row in lights2 {
	total_brightness += row.iter().sum::<usize>();
    }
    println!("{:?}", total_brightness);
}
