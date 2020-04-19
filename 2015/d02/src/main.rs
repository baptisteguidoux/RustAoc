
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;


#[derive(Debug, PartialEq)]
struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimensions {
    // input must be like "u32xu32xu32"
    fn new(input: &str) -> Dimensions {

	let sizes_vec: Vec<&str> = input.split('x').collect();
	
	let length = sizes_vec[0].parse().unwrap();
	let width = sizes_vec[1].parse().unwrap();
	let height = sizes_vec[2].parse().unwrap();	
	
	Dimensions { length, width, height }
    }

    fn lxw(&self) -> u32 {
	self.length * self.width
    }

    fn wxh(&self) -> u32 {
	self.width * self.height
    }

    fn hxl(&self) -> u32 {
	self.height * self.length
    }       

    fn surface(&self) -> u32 {
	2 * self.lxw() + 2 * self.wxh()	+ 2 * self.hxl()
    }

    fn smallest(&self) -> u32 {
	cmp::min(cmp::min(self.lxw(), self.wxh()), self.hxl())
    }

    fn required_ribbon(&self) -> u32 {
	let smallest = cmp::min(self.length, cmp::min(self.height, self.width));
	if smallest == self.length {
	    2 * self.length + 2 * cmp::min(self.height, self.width)
	} else if smallest == self.width {
	    2 * self.width + 2 * cmp::min(self.length, self.height)
	} else if smallest == self.height {
	    2 * self.height + 2 * cmp::min(self.width, self.length)
	} else {
	    panic!("IMPOSSIBLE")
	}
    }

    fn length_ribbon_bow(&self) -> u32 {
	self.length * self.height * self.width
    }
    
}


#[cfg(test)]
mod test {

    use crate::*;
    
    #[test]
    fn test_parse_string_to_dimensions() {
	assert_eq!(Dimensions::new("1x2x3"), Dimensions {length: 1, width: 2, height: 3});
	assert_eq!(Dimensions::new("29x16x22"), Dimensions {length: 29, width: 16, height: 22});
    }
  
    #[test]
    fn test_dimensions_calculate_surface() {
	let d1 = Dimensions {length: 2, width: 3, height: 4};
	assert_eq!(d1.surface(), 52);
	let d2 = Dimensions {length: 1, width: 1, height: 10};
	assert_eq!(d2.surface(), 42);
    }

    #[test]
    fn test_dimensions_smallest_side() {
	let d1 = Dimensions {length: 2, width: 3, height: 4};
	assert_eq!(d1.smallest(), 6);
	let d2 = Dimensions {length: 1, width: 1, height: 10};
	assert_eq!(d2.smallest(), 1);	
    }

    #[test]
    fn test_dimensions_required_ribbon() {
	let d1 = Dimensions {length: 2, width: 3, height: 4};
	assert_eq!(d1.required_ribbon(), 10);
	let d2 = Dimensions {length: 1, width: 1, height: 10};
	assert_eq!(d2.required_ribbon(), 4);
    }

    #[test]
    fn test_length_ribbon_bow() {
	let d1 = Dimensions {length: 2, width: 3, height: 4};
	assert_eq!(d1.length_ribbon_bow(), 24);
	let d2 = Dimensions {length: 1, width: 1, height: 10};
	assert_eq!(d2.length_ribbon_bow(), 10);	
    }

}

// (length l, width w, and height h)

fn main() {
    let file = File::open("data.txt").
	expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut res1: u32 = 0;
    let mut res2: u32 = 0;
    for line in buf_reader.lines() {
	let dim = Dimensions::new(&line.unwrap());
	res1 += dim.surface() + dim.smallest();
	res2 += dim.required_ribbon() + dim.length_ribbon_bow()
    }

    println!("res1: {}, res2: {}", res1, res2);
}
