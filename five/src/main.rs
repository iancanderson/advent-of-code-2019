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

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    GetInput,
    Print,
    EndOfProgram,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

fn char_to_parameter_mode(c: char) -> ParameterMode {
    match c {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        _ => panic!("invalid parameter mode")
    }
}

// Opcode integer is now more complicated..
fn int_to_opcode(int: i32) -> Opcode {
    if int == 1 {
        Opcode::Add
    } else if int == 2 {
        Opcode::Multiply
    } else if int == 3 {
        Opcode::GetInput
    } else if int == 4 {
        Opcode::Print
    } else {
        Opcode::EndOfProgram
    }
}

// Given the first value of an instruction, return
// (Opcode, Vec<ParameterMode>)
// The parameter mode vector represents the parameter modes
// of each parameter in the instruction.
fn parse_first_value(first_value: i32) -> (Opcode, Vec<ParameterMode>) {
    // Get lst two digits for the opcode
    // Convert to string, then take last two characters, then parse to int
    // Needs to work for both "2" and "1002"
    let mut as_string = first_value.to_string();

    if as_string.len() <= 2 {
        let opcode_string = as_string;
        let opcode_int: i32 = opcode_string.parse().unwrap();
        return (int_to_opcode(opcode_int), Vec::new());
    } else {
        let opcode_string = as_string.split_off(as_string.len() - 2);
        let opcode_int: i32 = opcode_string.parse().unwrap();

        let mut parameter_modes: Vec<ParameterMode> = Vec::new();

        // Rest of string is parameter modes, from right to left
        for parameter_mode_char in as_string.chars().rev() {
            parameter_modes.push(char_to_parameter_mode(parameter_mode_char));
        }

        return (int_to_opcode(opcode_int), parameter_modes);
    };
}

fn num_values_in_instruction(opcode: Opcode) -> usize {
    match opcode {
        Opcode::Add => 4,
        Opcode::Multiply => 4,
        Opcode::GetInput => 2,
        Opcode::Print => 2,
        Opcode::EndOfProgram => 1,
    }
}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut current_position = 0;

    loop {
        let (current_opcode, parameter_modes) = parse_first_value(program[current_position]);

        if let Opcode::EndOfProgram = current_opcode {
            break;
        }

        // Loop over operands
        let operands: Vec<i32> = vec![0, 1].iter().map(|&operand_offset| -> i32 {
            let operand = program[current_position + operand_offset + 1];

            let parameter_mode = parameter_modes.get(operand_offset).unwrap_or(&ParameterMode::Position);

            return match parameter_mode {
                ParameterMode::Position => program[operand as usize],
                ParameterMode::Immediate => operand,
            }
        }).collect();

        match current_opcode {
            Opcode::Add => {
                let result_location = program[current_position + 3] as usize;
                program[result_location] = operands[0] + operands[1];
            }
            Opcode::Multiply => {
                let result_location = program[current_position + 3] as usize;
                program[result_location] = operands[0] * operands[1];
            }
            Opcode::GetInput => panic!("todo"),
            Opcode::Print => {
                println!("{}", program[operands[0] as usize]);
            }
            Opcode::EndOfProgram => panic!("impossible"),
        };

        current_position += num_values_in_instruction(current_opcode);
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

    // #[test]
    // fn opcode_3_and_4() {
    //     let program = vec![1,1,1,4,99,5,6,0,99];
    //     let answer = run_intcode(program);
    //     assert_eq!(answer, vec![30,1,1,4,2,5,6,0,99]);
    // }

    #[test]
    fn parameter_modes() {
        let program = vec![1002,4,3,4,33];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_parse_first_value() {
        assert_eq!(
            parse_first_value(1002),
            (
                Opcode::Multiply,
                vec![ParameterMode::Position, ParameterMode::Immediate],
            )
        );
    }
}
