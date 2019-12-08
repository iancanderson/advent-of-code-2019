fn main() {
    for noun in 00..100 {
        for verb in 00..100 {
            let mut program = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,6,23,27,1,6,27,31,2,31,9,35,1,35,6,39,1,10,39,43,2,9,43,47,1,5,47,51,2,51,6,55,1,5,55,59,2,13,59,63,1,63,5,67,2,67,13,71,1,71,9,75,1,75,6,79,2,79,6,83,1,83,5,87,2,87,9,91,2,9,91,95,1,5,95,99,2,99,13,103,1,103,5,107,1,2,107,111,1,111,5,0,99,2,14,0,0];

            program[1] = noun;
            program[2] = verb;

            if run_intcode(program)[0] == 19690720 {
                println!("noun: {}", noun);
                println!("verb: {}", verb);
                println!("100 * noun + verb: {}", 100 * noun + verb);
            }
        }
    }
}

enum Opcode {
    Add,
    Multiply,
    EndOfProgram,
}

fn int_to_opcode(int: i32) -> Opcode {
    if int == 1 {
        Opcode::Add
    } else if int == 2 {
        Opcode::Multiply
    } else {
        Opcode::EndOfProgram
    }
}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut current_position = 0;

    loop {
        let current_opcode = int_to_opcode(program[current_position]);
        if let Opcode::EndOfProgram = current_opcode {
            break;
        }

        let operand1_location = program[current_position + 1] as usize;
        let operand2_location = program[current_position + 2] as usize;
        let operand1 = program[operand1_location];
        let operand2 = program[operand2_location];

        let result = match current_opcode {
            Opcode::Add => operand1 + operand2,
            Opcode::Multiply => operand1 * operand2,
            Opcode::EndOfProgram => panic!("impossible"),
        };

        let result_location = program[current_position + 3] as usize;

        program[result_location] = result;

        current_position += 4;
    }

    return program;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_program_one() {
        let program = vec![1,0,0,0,99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![2,0,0,0,99]);
    }

    #[test]
    fn example_program_two() {
        let program = vec![2,3,0,3,99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![2,3,0,6,99]);
    }

    #[test]
    fn example_program_three() {
        let program = vec![2,4,4,5,99,0];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn example_program_four() {
        let program = vec![1,1,1,4,99,5,6,0,99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![30,1,1,4,2,5,6,0,99]);
    }
}
