#[cfg(test)]
mod tests {

    use crate::*;
    
    #[test]
    fn test_run_incode_program() {
        let mut a = [1,9,10,3,2,3,11,0,99,30,40,50];
        run_incode_program(&mut a);
        assert_eq!(a, [3500,9,10,70,2,3,11,0,99,30,40,50]);

        let mut b = [1,0,0,0,99];
        run_incode_program(&mut b);
        assert_eq!(b, [2,0,0,0,99]);

        let mut c = [2,3,0,3,99];
        run_incode_program(&mut c);
        assert_eq!(c, [2,3,0,6,99]);

        let mut d = [2,4,4,5,99,0];
        run_incode_program(&mut d);        
        assert_eq!(d, [2,4,4,5,99,9801]);

        let mut e = [1,1,1,4,99,5,6,0,99];
        run_incode_program(&mut e);
        assert_eq!(e, [30,1,1,4,2,5,6,0,99]);
    }
}

fn run_incode_program(program: &mut [i32]) {

    let mut pointer = 0; // we start at 0

    loop {
        let opcode = &program[pointer];
        match opcode {
            1 => {
                let operand1_pos = program[pointer + 1] as usize;
                let operand2_pos = program[pointer + 2] as usize;
                let store_position = program[pointer + 3] as usize;
                program[store_position] = &program[operand1_pos] + &program[operand2_pos];
            },
            2 => {
                let operand1_pos = program[pointer + 1] as usize;
                let operand2_pos = program[pointer + 2] as usize;
                let store_position = program[pointer + 3] as usize;
                program[store_position] = &program[operand1_pos] * &program[operand2_pos];
            },
            99 => break,
            _ => println!("Opcode {} not understood!", opcode)
        }

        pointer += 4;
    }

}

fn main() {

    let input = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,6,23,27,1,27,5,31,2,9,31,35,1,5,35,39,2,6,39,43,2,6,43,47,1,5,47,51,2,9,51,55,1,5,55,59,1,10,59,63,1,63,6,67,1,9,67,71,1,71,6,75,1,75,13,79,2,79,13,83,2,9,83,87,1,87,5,91,1,9,91,95,2,10,95,99,1,5,99,103,1,103,9,107,1,13,107,111,2,111,10,115,1,115,5,119,2,13,119,123,1,9,123,127,1,5,127,131,2,131,6,135,1,135,5,139,1,139,6,143,1,143,6,147,1,2,147,151,1,151,5,0,99,2,14,0,0];


    // PART ONE
    let mut input1 = input.clone();
    
    // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    input1[1] = 12;
    input1[2] = 2;

    run_incode_program(&mut input1);

    println!("The value at position 0 is {}", input1[0]);


    // PART TWO
    for noun in 0..100 {
        for verb in 0..100 {
            let mut input2 = input.clone();
            input2[1] = noun;
            input2[2] = verb;

            run_incode_program(&mut input2);

            if input2[0] == 19690720 {
                println!{"noun: {}, verb: {}", noun, verb};
                println!("100 * noun + verb: {}", 100 * noun + verb);
                break;
            }
        }
    }        
}

