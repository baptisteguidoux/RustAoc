use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::convert::TryInto;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_path_to_vec_positions() {
        let path = "R2,U1,L2,U1,D2";
        let vec_pos = vec![
            (1, 0),
            (2, 0),
            (2, 1),
            (1, 1),
            (0, 1),
            (0, 2),
            (0, 1),
            (0, 0),
        ];
        assert_eq!(path_to_vec_positions(path), vec_pos);
    }

    #[test]
    fn test_find_intersections() {
        let crossed = find_intersections(&path_to_vec_positions("R8,U5,L5,D3"),
					 &path_to_vec_positions("U7,R6,D4,L4"));
        let mut res = HashSet::new();
        res.insert((3, 3));
        res.insert((6, 5));

        assert_eq!(crossed, res);
    }

    #[test]
    fn test_calc_dist() {
        assert_eq!(calc_dist((0, 0), (0, 0)), 0);
        assert_eq!(calc_dist((1, 0), (0, 0)), 1);
        assert_eq!(calc_dist((3, 3), (0, 0)), 6);
        assert_eq!(calc_dist((5, 2), (3, 1)), 3);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            part_one(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            part_one(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn test_number_of_steps_to_intersection() {
	let wire1 = path_to_vec_positions("R8,U5,L5,D3");
	let wire2 = path_to_vec_positions("U7,R6,D4,L4");

	let inter = (3, 3);
	let wire1_steps = number_of_steps_to_intersection(&wire1, inter);
	let wire2_steps = number_of_steps_to_intersection(&wire2, inter);
	
	assert_eq!(wire1_steps, 20);
	assert_eq!(wire2_steps, 20);

	let inter = (6, 5);
	let wire1_steps = number_of_steps_to_intersection(&wire1, inter);
	let wire2_steps = number_of_steps_to_intersection(&wire2, inter);
	
	assert_eq!(wire1_steps, 15);
	assert_eq!(wire2_steps, 15);	
    }

    #[test]
    fn test_fewest_steps_to_intersection() {
	let wire1 = "R8,U5,L5,D3";
	let wire2 = "U7,R6,D4,L4";	
	let steps = fewest_steps_to_intersection(wire1, wire2);

	assert_eq!(steps, 30);

	let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
	let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83";	
	let steps = fewest_steps_to_intersection(wire1, wire2);

	assert_eq!(steps, 610);

	let wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
	let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";	
	let steps = fewest_steps_to_intersection(wire1, wire2);

	assert_eq!(steps, 410);	
    }
}


fn path_to_vec_positions(path: &str) -> Vec<(i32, i32)> {
    let mut last_pos = (0, 0);
    let mut positions: Vec<(i32, i32)> = Vec::new();

    for dir in path.split(",").collect::<Vec<&str>>().iter() {

	let (letter, nums) = dir.split_at(1);
        let dist = nums.parse::<i32>().unwrap();

        match letter {
            "R" => {
                for _i in 0..dist {
                    last_pos.0 += 1;
                    positions.push(last_pos);
                }
            },
            "L" => {
                for _i in 0..dist {
                    last_pos.0 -= 1;
                    positions.push(last_pos);
                }
            },
            "U" => {
                for _i in 0..dist {
                    last_pos.1 += 1;
                    positions.push(last_pos);
                }
            },
            "D" => {
                for _i in 0..dist {
                    last_pos.1 -= 1;
                    positions.push(last_pos);
                }
            },
            _ => panic!("instruction unknown"),
        }	
    }
    positions
}

fn find_intersections(movs1: &Vec<(i32, i32)>, movs2: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {

    let movs1: HashSet<_> = movs1.iter().cloned().collect();
    let movs2: HashSet<_> = movs2.iter().cloned().collect();

    movs1.intersection(&movs2).cloned().collect()
}

fn calc_dist(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    i32::abs(pos1.0 - pos2.0) + i32::abs(pos1.1 - pos2.1)
}

fn part_one(path1: &str, path2: &str) -> i32 {
    let inters = find_intersections(&path_to_vec_positions(path1),
				    &path_to_vec_positions(path2));
    let mut distances = Vec::<i32>::new();
    for cross in inters.iter() {
        distances.push(calc_dist(*cross, (0, 0)));
    }
    let smallest_dist = distances.iter().min();
    match smallest_dist {
        Some(min) => *min,
        None => panic!("No smallest distance found!"),
    }
}

fn number_of_steps_to_intersection(wire: &Vec<(i32, i32)>, intersection: (i32, i32)) -> isize {
    match wire.iter().position(|&s| s == intersection) {
	Some(pos) => (pos + 1).try_into().unwrap(),
	None => -1,
    }
}

fn fewest_steps_to_intersection(steps1: &str, steps2: &str) -> isize {
    let wire1 = path_to_vec_positions(steps1);
    let wire2 = path_to_vec_positions(steps2);
    let intersections = find_intersections(&wire1, &wire2);

    let mut fewest_steps = -1;
    for &intersection in intersections.iter() {
	let wire1_steps = number_of_steps_to_intersection(&wire1, intersection);
	let wire2_steps = number_of_steps_to_intersection(&wire2, intersection);
	let combined_steps = wire1_steps + wire2_steps;
	if fewest_steps == -1 {
	    fewest_steps = combined_steps;
	} else{
	    if fewest_steps > combined_steps {
		fewest_steps = combined_steps;
	    }
	}
    }

    fewest_steps
}

fn main() {
    let input_file = File::open("input.txt").expect("could not open input.txt");

    let mut paths = [String::new(), String::new()];

    for (i, line) in io::BufReader::new(input_file).lines().enumerate() {
        paths[i] = line.unwrap();
    }

    let res1 = part_one(&paths[0], &paths[1]);
    println!("{}", res1);

    let res2 = fewest_steps_to_intersection(&paths[0], &paths[1]);
    println!("{}", res2);
}
