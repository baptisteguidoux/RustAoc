use std::fs;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_string_to_parentheses() {
	assert_eq!(
	    string_to_parentheses("(())"),
	    vec![Parenthese::Open, Parenthese::Open, Parenthese::Close, Parenthese::Close]
	);
	
	assert_eq!(
	    string_to_parentheses("()()"),
	    vec![Parenthese::Open, Parenthese::Close, Parenthese::Open, Parenthese::Close]
	);
    }

    #[test]
    fn test_vec_parentheses_to_vec_int() {
	assert_eq!(
	    vec_parentheses_to_vec_int(vec![Parenthese::Open, Parenthese::Open, Parenthese::Close, Parenthese::Close]),
	    vec![1, 1, -1, -1]
	);
	assert_eq!(
	    vec_parentheses_to_vec_int(vec![Parenthese::Open, Parenthese::Close, Parenthese::Open, Parenthese::Close]),
	    vec![1, -1, 1, -1]
	);
    }

    #[test]
    fn test_sum_vec_int() {
	assert_eq!(sum_vec_int(vec![1, 1, -1, -1]), 0);
	assert_eq!(sum_vec_int(vec![1, -1, 1, -1]), 0);
	assert_eq!(sum_vec_int(vec![-1, -1, -1]), -3);
	assert_eq!(sum_vec_int(vec![-1, -1, 1, 1, 1, 1, 1]), 3);
    }

    #[test]
    fn test_question1() {
	assert_eq!(question1("(())"), 0);
	assert_eq!(question1("()()"), 0);
	assert_eq!(question1("((("), 3);
	assert_eq!(question1("(()(()("), 3);
	assert_eq!(question1("))((((("), 3);
	assert_eq!(question1("())"), -1);
	assert_eq!(question1("))("), -1);
	assert_eq!(question1(")))"), -3);
	assert_eq!(question1(")())())"), -3);
    }

    #[test]
    fn test_question2() {
	assert_eq!(question2(")"), 1);
	assert_eq!(question2("()())"), 5);
    }
}

#[derive(PartialEq, Debug)]
enum Parenthese {
    Open,
    Close
}

fn string_to_parentheses(input: &str) -> Vec<Parenthese> {
    let mut result: Vec<Parenthese> = Vec::new();

    for char in input.chars() {
	if char == '(' {
	    result.push(Parenthese::Open);
	} else if char == ')' {
	    result.push(Parenthese::Close);
	}
    }
    
    result
}

fn vec_parentheses_to_vec_int(input: Vec<Parenthese>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    
    for parenthese in &input {
	if *parenthese == Parenthese::Open {
	    result.push(1);
	} else if *parenthese == Parenthese::Close {
	    result.push(-1);
	} else {
	    panic!("glatorg?");
	}
    }

    result
}

fn sum_vec_int(input: Vec<i32>)-> i32 {
    input.iter().sum()
}

fn question1(input: &str) -> i32 {
    sum_vec_int(vec_parentheses_to_vec_int(string_to_parentheses(input)))
}

fn question2(input: &str) -> usize {

    let mut result = 0;

    for (idx, val) in vec_parentheses_to_vec_int(string_to_parentheses(input))
	.iter().enumerate() {
	result += val;
	if result <= -1 {
	    return idx + 1
	}
    }

    panic!("Never went to the basement");
}

fn main() {

    let input = fs::read_to_string("input.txt")
	.expect("Could not open file.");

    println!("result 1: {}", question1(&input));
    println!("result 2: {}", question2(&input));
}


