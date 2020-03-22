
#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_is_length() {
        assert!(is_length(27575, 5));
        assert!(is_length(1, 1));
        assert!(is_length(1000000, 7));
        assert!(! is_length(112415, 2));        
    }

    #[test]
    fn test_is_in_range() {
        assert!(is_in_range(20, 0, 100));
        assert!(! is_in_range(105, 0, 100));
        assert!(is_in_range(555, 500, 1000));               
    }

    #[test]
    fn test_has_two_adjacent_identical_symbols() {
        assert!(has_two_adjacent_identical_symbols(122345));
        assert!(! has_two_adjacent_identical_symbols(123456));
        assert!(has_two_adjacent_identical_symbols(1234456));        
    }

    #[test]
    fn test_sequence_never_decrease() {
        assert!(sequence_never_decrease(111123));
        assert!(sequence_never_decrease(135679));
        assert!(! sequence_never_decrease(1231456));
    }

    #[test]
    fn test_meet_criteria() {
        assert!(meet_criteria(111111));
        assert!(! meet_criteria(223450));
        assert!(! meet_criteria(123789));
    }
}


fn is_length<T: ToString>(value: T, length: usize) -> bool {
    value.to_string().len() == length
}

fn is_in_range<T: PartialOrd>(value: T, lower_bound: T, upper_bound: T) -> bool {
    lower_bound <= value && value <= upper_bound
}

fn has_two_adjacent_identical_symbols<T: ToString>(input: T) -> bool {

    let mut previous_char = input.to_string().chars().next().unwrap();
    
    for (idx, char) in input.to_string().char_indices() {
        if idx == 0 {
            continue;
        }
        if char == previous_char {
            return true;
        }
        previous_char = char;
    }
    false
}

fn sequence_never_decrease<T: ToString>(input: T) -> bool {

    let mut previous_symbol = input.to_string().chars().next().unwrap();

    // it works if comparison holds when converted to ascii codes
    
    for (idx, symbol) in input.to_string().char_indices() {
        if idx == 0 {
            continue;
        }

        if symbol < previous_symbol {
            return false;
        }
        previous_symbol = symbol;
    }
    true
}


const PASSWORD_LENGTH: usize = 6;
const PASSWORD_LOWER_RANGE: i32 = 109165;
const PASSWORD_UPPER_RANGE: i32 = 576723;

fn meet_criteria(input: i32) -> bool {
    is_length(input, PASSWORD_LENGTH) && is_in_range(input, PASSWORD_LOWER_RANGE, PASSWORD_UPPER_RANGE)
        && has_two_adjacent_identical_symbols(input) && sequence_never_decrease(input)
}

fn main() {

    let mut count = 0;
    
    for password in PASSWORD_LOWER_RANGE..PASSWORD_UPPER_RANGE+1 {
        if meet_criteria(password) {
            count+= 1;
        }
    }
    
    println!("count: {}", count);
}

