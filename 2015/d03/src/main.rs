use std::fs;
use std::ops::Add;
use std::ops::Sub;
use std::collections::HashSet;


#[derive(Debug, PartialEq)]
enum Move {
    Up,
    Right,
    Down,
    Left
}

impl Move {
    fn new(mv: char) -> Option<Move> {
	match mv {
	    '^' => Some(Move::Up),
	    '>' => Some(Move::Right),
	    'v' => Some(Move::Down),
	    '<' => Some(Move::Left),
	    '\n' => None, // should be in a parser, but will do the trick
	    _ => panic!("character {chara} unknown", chara = mv)    
	}
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i32, i32);

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
	Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
	Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Position {
    fn moves(self, move_: Move) -> Position {
	match move_ {
	    Move::Up => Position(self.0, self.1 + 1),
	    Move::Right => Position(self.0 + 1, self.1),
	    Move::Down => Position(self.0, self.1 - 1),
	    Move::Left => Position(self.0 - 1, self.1),
	}
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_move_new() {
	assert_eq!(Move::new('^'), Some(Move::Up));
	assert_eq!(Move::new('>'), Some(Move::Right));
	assert_eq!(Move::new('v'), Some(Move::Down));
	assert_eq!(Move::new('<'), Some(Move::Left));
    }

    #[test]
    fn test_position_operations() {
	assert_eq!(Position(0, 0) + Position(1, 1), Position(1, 1));
	assert_eq!(Position(1, 0) + Position(1, 1), Position(2, 1));
	assert_eq!(Position(0, 1) + Position(0, 1), Position(0, 2));

	assert_eq!(Position(0, 0) - Position(1, 1), Position(-1, -1));
	assert_eq!(Position(1, 0) - Position(1, 1), Position(0, -1));
	assert_eq!(Position(2, 3) - Position(0, 1), Position(2, 2));
    }

    #[test]
    fn test_position_after_move() {
	assert_eq!(Position(0, 0).moves(Move::Up), Position(0, 1));
	assert_eq!(Position(1, -2).moves(Move::Right), Position(2, -2));
	assert_eq!(Position(4, 7).moves(Move::Down), Position(4, 6));
	assert_eq!(Position(-1, -3).moves(Move::Left), Position(-2, -3));
    }
}
    

fn main() {
    let input: String = fs::read_to_string("data.txt").
	expect("Could not open file").parse().expect("could not parse file");
    
    // Part 1
    let mut current_position = Position(0, 0);
    let mut visited_houses = HashSet::new();
    visited_houses.insert(current_position);
    
    for mov in input.chars() {
	match Move::new(mov) {
	    Some(mov) => {
		current_position = current_position.moves(mov);
		visited_houses.insert(current_position);
	    }
	    None => ()
	}
    }
    println!("Part 1: {} visited houses", visited_houses.len());

    // Part 2
    let mut santa_curpos = Position(0, 0);
    let mut robosanta_curpos = Position(0, 0);
    visited_houses = HashSet::new();
    visited_houses.insert(santa_curpos);

    // Not using enumerate to have (i, mov), because we match against an Option
    let mut i = 1;
    for mov in input.chars() {
	match Move::new(mov) {
	    Some(mov) => {
		if i % 2 == 1 {
		    santa_curpos = santa_curpos.moves(mov);
		    visited_houses.insert(santa_curpos);
		    i += 1;
		} else {
		    robosanta_curpos = robosanta_curpos.moves(mov);
		    visited_houses.insert(robosanta_curpos);
		    i += 1;
		}
	    }
	    None => ()
	}
    }
    println!("Part 2: {} visited houses", visited_houses.len());
}
