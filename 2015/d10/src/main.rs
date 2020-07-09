
fn group_contiguous<T: PartialEq + Copy>(to_group: Vec<T>) -> Vec<Vec<T>> {
    let mut res = Vec::new();

    if to_group.len() == 0 {
	return res;
    }

    let mut sub_group = Vec::new();
    let mut previous_item = to_group[0];
    for item in to_group {
	if item == previous_item {
	    sub_group.push(item);
	    previous_item = item;
	} else {
	    res.push(sub_group);
	    sub_group = Vec::new();
	    sub_group.push(item);
	    previous_item = item;
	}
    }
    res.push(sub_group); //last sub group
    res
}

fn calc_looknsay_step(input: &str) -> String {
    // string to vec 
    let str_chars: Vec<char> = input.chars().collect();
    let grouped_chars = group_contiguous(str_chars);
    let mut res = String::new();

    for chars_group in grouped_chars {
	res.push_str(&chars_group.len().to_string());
	res.push_str(&chars_group[0].to_string());
    }
    
    res
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_group_contiguous() {
	assert_eq!(
	    group_contiguous(vec![1, 1, 1, 2, 3, 3, 4, 5, 5, 5]),
	    vec![vec![1, 1, 1], vec![2], vec![3, 3], vec![4], vec![5, 5, 5]]
	);

	assert_eq!(
	    group_contiguous(vec![1, 2, 3, 4, 5]),
	    vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
	);

	assert_eq!(
	    group_contiguous(vec![1, 2, 2, 3, 3, 4, 4, 5]),
	    vec![vec![1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5]]
	);

	assert_eq!(group_contiguous(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn test_calc_looknsay_step() {
	assert_eq!(
	    calc_looknsay_step("1"),
	    "11"
	);
	assert_eq!(
	    calc_looknsay_step("11"),
	    "21"
	);
	assert_eq!(
	    calc_looknsay_step("21"),
	    "1211"
	);
	assert_eq!(
	    calc_looknsay_step("1211"),
	    "111221"
	);
	assert_eq!(
	    calc_looknsay_step("111221"),
	    "312211"
	);
    }
}


fn main() {

    let mut input = String::from("1113122113");
    
    for i in 0..50 {
	input = calc_looknsay_step(&input);
    }

    println!("{}", input.len());
}
