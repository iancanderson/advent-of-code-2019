use std::io::{stdin,stdout,Write};
use std::fmt;

fn main() {
    let program = vec![3,8,1001,8,10,8,105,1,0,0,21,30,51,72,81,94,175,256,337,418,99999,3,9,101,5,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,1001,9,2,9,1002,9,5,9,4,9,99,3,9,1002,9,4,9,101,4,9,9,102,5,9,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,4,9,99,3,9,102,3,9,9,1001,9,4,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,99];

    run_intcode(program);
}

fn max_thruster_signal(program: Vec<i32>) -> i32 {
    return 1;
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    GetInput,
    Print,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    EndOfProgram,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
        _ => panic!("invalid parameter mode: {}", c)
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
    } else if int == 5 {
        Opcode::JumpIfTrue
    } else if int == 6 {
        Opcode::JumpIfFalse
    } else if int == 7 {
        Opcode::LessThan
    } else if int == 8 {
        Opcode::Equals
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
        Opcode::JumpIfTrue => 3,
        Opcode::JumpIfFalse => 3,
        Opcode::LessThan => 4,
        Opcode::Equals => 4,
        Opcode::EndOfProgram => 1,
    }
}

fn get_input_as_int() -> i32 {
    let mut s = String::new();
    println!("Enter input:");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s.parse().expect("Input was not a valid integer");
}

fn resolve_operands(program: &Vec<i32>, current_position: usize, parameter_modes: Vec<ParameterMode>, num_operands: usize) -> Vec<i32> {
    // Loop over operands
    let operands: Vec<i32> = (0..num_operands).map(|operand_offset| -> i32 {
        let index = current_position + operand_offset + 1;
        let operand = program[index];

        let parameter_mode = parameter_modes.get(operand_offset).unwrap_or(&ParameterMode::Position);

        let resolved_operand = match parameter_mode {
            ParameterMode::Position => program[operand as usize],
            ParameterMode::Immediate => operand,
        };

        // println!("raw operand: {}", operand);
        // println!("resolved operand: {}", resolved_operand);

        return resolved_operand;
    }).collect();

    return operands;
}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut current_position = 0;

    loop {
        let (current_opcode, parameter_modes) = parse_first_value(program[current_position]);

        println!("Running {} at position {}", current_opcode, current_position);

        match current_opcode {
            Opcode::Add => {
                let result_location = program[current_position + 3] as usize;
                let operands = resolve_operands(&program, current_position, parameter_modes, 2);
                program[result_location] = operands[0] + operands[1];
            }
            Opcode::Multiply => {
                let result_location = program[current_position + 3] as usize;
                let operands = resolve_operands(&program, current_position, parameter_modes, 2);
                program[result_location] = operands[0] * operands[1];
            }
            Opcode::GetInput => {
                let input = get_input_as_int();
                let operand = program[current_position + 1];
                program[operand as usize] = input;
            }
            Opcode::Print => {
                let operand = program[current_position + 1];
                println!("Out: {}", program[operand as usize]);
            }
            Opcode::JumpIfTrue => {
                let operands = resolve_operands(&program, current_position, parameter_modes, 2);

                if operands[0] != 0 {
                    current_position = operands[1] as usize;
                    continue;
                }
            }
            Opcode::JumpIfFalse => {
                let operands = resolve_operands(&program, current_position, parameter_modes, 2);

                if operands[0] == 0 {
                    current_position = operands[1] as usize;
                    continue;
                }
            }
            Opcode::LessThan => {
                let operands = resolve_operands(&program, current_position, parameter_modes, 2);
                let storage_location = program[current_position + 3] as usize;

                program[storage_location] =
                    if operands[0] < operands[1] {
                        1
                    } else {
                        0
                    }
            }
            Opcode::Equals => {
                let operands = resolve_operands(&program, current_position, parameter_modes, 2);
                let storage_location = program[current_position + 3] as usize;

                program[storage_location] =
                    if operands[0] == operands[1] {
                        1
                    } else {
                        0
                    }
            }
            Opcode::EndOfProgram => {
                break;
            }
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

    #[test]
    fn test_parameter_modes() {
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

    #[test]
    fn test_jump_if_true_when_false() {
        let program = vec![105, 0, 7, 1, 0, 0, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![210, 0, 7, 1, 0, 0, 0, 99]);
    }

    #[test]
    fn test_jump_if_true_when_true() {
        let program = vec![1105, 1, 7, 1, 0, 0, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![1105, 1, 7, 1, 0, 0, 0, 99]);
    }

    #[test]
    fn test_jump_if_false_when_false() {
        let program = vec![1106, 0, 7, 1, 0, 0, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![1106, 0, 7, 1, 0, 0, 0, 99]);
    }

    #[test]
    fn test_jump_if_false_when_true() {
        let program = vec![106, 1, 7, 1, 0, 0, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![212, 1, 7, 1, 0, 0, 0, 99]);
    }

    #[test]
    fn test_less_than_when_true() {
        let program = vec![107, 1, 2, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![1, 1, 2, 0, 99]);
    }

    #[test]
    fn test_less_than_when_false() {
        let program = vec![107, 2, 2, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![0, 2, 2, 0, 99]);
    }

    #[test]
    fn test_equals_when_true() {
        let program = vec![108, 2, 2, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![1, 2, 2, 0, 99]);
    }

    #[test]
    fn test_equals_when_false() {
        let program = vec![108, 1, 2, 0, 99];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![0, 1, 2, 0, 99]);
    }

    #[test]
    fn test_max_thruster_example_one() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(max_thruster_signal(program), 43210);
    }
}
