use std::io::{BufRead, BufReader};
use std::fs::File;


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_get_module_fuel_need() {
        assert_eq!(get_module_fuel_need(12), 2);
        assert_eq!(get_module_fuel_need(14), 2);
        assert_eq!(get_module_fuel_need(1969), 654);
        assert_eq!(get_module_fuel_need(100756), 33583);
    }

    #[test]
    fn test_get_module_fuel_need_rec() {
	assert_eq!(get_module_fuel_need_rec(0, 0), 0);
	assert_eq!(get_module_fuel_need_rec(14, 0), 2);
	assert_eq!(get_module_fuel_need_rec(1969, 0), 966);
	assert_eq!(get_module_fuel_need_rec(100756, 0), 50346);
    }
}

fn get_module_fuel_need(module_mass: i32) -> i32 {
    (module_mass / 3) - 2
}

// Takes into account the fuel required by adding the fuel...
fn get_module_fuel_need_rec(module_mass: i32, total_required_fuel: i32) -> i32 {

    let extra_fuel_needed = (module_mass / 3) - 2;

    if extra_fuel_needed > 0 {
	get_module_fuel_need_rec(extra_fuel_needed, total_required_fuel + extra_fuel_needed)
    } else {
	total_required_fuel
    }
}

fn main() {

    let reader = BufReader::new(File::open("input.txt")
	.expect("cannnot open file"));

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in reader.lines() {
	match line {
	    Ok(li) => {
		let value = li.parse::<i32>().unwrap();
		sum1 += get_module_fuel_need(value);
		sum2 += get_module_fuel_need_rec(value, 0);
	    },
	    _ => println!("Couldn't parse line"),
	}
    }
    
    println!("{}, {}", sum1, sum2);
}

