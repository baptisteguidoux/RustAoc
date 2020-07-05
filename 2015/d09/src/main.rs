use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


#[derive(std::fmt::Debug)]
struct Journey(String, String);

impl Journey {
    fn new(s: &str, e: &str) -> Journey {
	Journey(s.to_string(), e.to_string())
    }
}

impl PartialEq for Journey {
    fn eq(&self, other: &Self) -> bool {
	(self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Journey {}

impl Hash for Journey {
    fn hash<H: Hasher>(&self, state: &mut H) {
	let a = self.0.as_bytes().iter().map(|&b| b as usize).sum::<usize>();
	let b = self.1.as_bytes().iter().map(|&b| b as usize).sum::<usize>();
	(a + b).hash(state);
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_eq() {
	let dist1 = Journey::new("London", "Dublin");
	let dist2 = Journey::new("Dublin", "London");

	assert_eq!(dist1, dist2);

	let dist3 = Journey::new("London", "Dubai");

	assert!(dist1 != dist3);
    }
    
    #[test]
    fn test_hash() {
	let dist1 = Journey::new("London", "Dublin");
	let dist2 = Journey::new("Dublin", "London");

	let mut hasher1= DefaultHasher::new();
	let mut hasher2= DefaultHasher::new();	
	
	dist1.hash(&mut hasher1);
	dist2.hash(&mut hasher2);
	let hash1 = hasher1.finish();
	assert_eq!(hash1, hasher2.finish());

	let dist3 = Journey::new("London", "Dubai");
	let mut hasher3= DefaultHasher::new();
	dist3.hash(&mut hasher3);
	assert!(hash1 != hasher3.finish());
    }
}


fn permutations_heap<T: Clone>(n: usize, mut vector: Vec<T>) -> Vec<Vec<T>> {

    let mut res = Vec::new();
    res.push(vector.clone());

    let mut c = vec![0; n];

    let mut i = 0;
    while i < n {
	if c[i] < i {
	    if i % 2 == 0 {
		vector.swap(0, i);
	    } else {
		vector.swap(c[i], i);
	    }
	    res.push(vector.clone());
	    c[i] += 1;
	    i = 0;
	} else {
	    c[i] = 0;
	    i += 1;
	}
    }
    res
} 


fn main() {

    // let test_inputs = vec!["London to Dublin = 464",
    // 			   "London to Belfast = 518",
    // 			   "Dublin to Belfast = 141"];

    let file = File::open("data.txt").expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut distances = HashMap::new();
    let mut locations = HashSet::new();
    
    //for dist in test_inputs {
    for line in buf_reader.lines() {
	let dist = line.unwrap();
	let split_dist: Vec<&str> = dist.split(" = ").collect();
	let dist = split_dist[1].parse::<usize>().unwrap();
	let src_dst: Vec<&str> = split_dist[0].split(" to ").collect();
	let src = src_dst[0];
	let dst = src_dst[1];
	let journey = Journey::new(src, dst);
	distances.insert(journey, dist);

	locations.insert(src.to_string());
	locations.insert(dst.to_string());
    }

    let locations: Vec<String> = locations.drain().collect();

    let mut min = std::usize::MAX;
    let mut max = 0;
    for v in permutations_heap(locations.len(), locations) {
	//println!("{:?}", v);
	let sum_dist = v.iter().zip(&v[1..v.len()])
	    .map(|(a, b)| distances.get(&Journey::new(a, b)).unwrap()).sum::<usize>();
	//println!("{}", sum_dist);
	min = cmp::min(min, sum_dist);
	max = cmp::max(max, sum_dist);
    }

    println!("{}", min);
    println!("{}", max);
}
