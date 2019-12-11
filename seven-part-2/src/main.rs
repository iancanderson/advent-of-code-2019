use std::io::{stdin,stdout,Write};
use std::fmt;
use std::collections::HashMap;
use itertools::{Itertools, Permutations};

fn main() {
    let program = vec![3,8,1001,8,10,8,105,1,0,0,21,30,51,72,81,94,175,256,337,418,99999,3,9,101,5,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,1001,9,2,9,1002,9,5,9,4,9,99,3,9,1002,9,4,9,101,4,9,9,102,5,9,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,4,9,99,3,9,102,3,9,9,1001,9,4,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,99];

    println!("Max thruster signal: {}", max_thruster_signal(&program));
}

fn thruster_signal(phase_setting_sequence: &Vec<&i32>, program: &Vec<i32>) -> i32 {
    let input_signal = 0;

    return phase_setting_sequence.iter().fold(input_signal, |signal, phase_setting| {
        // Each iteration represents a program execution on a particular amplifier,
        // with its own input signal and phase setting
        let inputs = vec![**phase_setting, signal];

        let program_copy = program.clone();

        let (_, output_signal) = run_intcode_headless(program_copy, inputs);

        // Value returned from this closure is the output signal from this amplifier
        return output_signal.expect("Should be an output signal");
    });
}

fn max_thruster_signal(program: &Vec<i32>) -> i32 {
    // Loop over all permutations of [0,1,2,3,4]
    // Find permutation with max thruster signal

    return vec![0,1,2,3,4].iter().permutations(5).map(|phase_setting_sequence| {
        let signal = thruster_signal(&phase_setting_sequence, program);
        println!("Sequence {:?} gives output signal: {}", phase_setting_sequence, signal);
        return signal;
    }).max().expect("There will definitely be a maximum");
}

fn thruster_signal_with_feedback(phase_setting_sequence: &Vec<&i32>, program: &Vec<i32>) -> i32 {
    let input_signal = 0;
    let mut programs = HashMap::new();

    for phase_setting in phase_setting_sequence {
        programs.insert(phase_setting, program.clone());
    }

    let amplifier_e_first_output = phase_setting_sequence.iter().fold(input_signal, |signal, phase_setting| {
        // Each iteration represents a program execution on a particular amplifier,
        // with its own input signal and phase setting
        let inputs = vec![**phase_setting, signal];

        let amplifier_program = programs.get(phase_setting).expect("No program for phase setting");

        let (new_program, output_signal) = run_intcode_headless(amplifier_program.to_vec(), inputs);

        programs.insert(phase_setting, new_program);

        // Value returned from this closure is the output signal from this amplifier
        return output_signal.expect("Should be an output signal");
    });

    return amplifier_e_first_output;
}

fn max_thruster_signal_with_feedback(program: &Vec<i32>) -> i32 {
    return vec![5,6,7,8,9].iter().permutations(5).map(|phase_setting_sequence| {
        // First, run each amplifier until 
        let signal = thruster_signal_with_feedback(&phase_setting_sequence, program);
        println!("Sequence {:?} gives output signal: {}", phase_setting_sequence, signal);
        return signal;
    }).max().expect("There will definitely be a maximum");
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
    return run_intcode_headless(program, Vec::new()).0;
}

// Run a program
//
// inputs: represents values that will be consumed by GetInput instruction. if all input values
// are consumed, the program will get input interactively from the user
//
// Returns optional last_output_value: will be the value of the last Output instruction
fn run_intcode_headless(mut program: Vec<i32>, inputs: Vec<i32>) -> (Vec<i32>, Option<i32>) {
    let mut current_position = 0;
    let mut current_input_index = 0;
    let mut last_output_value = None;

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
                let resolved_input =
                    if let Some(input) = inputs.get(current_input_index) {
                        current_input_index += 1;
                        *input
                    } else {
                        get_input_as_int()
                    };

                println!("Resolved input: {}", resolved_input);

                let operand = program[current_position + 1];
                program[operand as usize] = resolved_input;
            }
            Opcode::Print => {
                let operand = program[current_position + 1];
                let output = program[operand as usize];
                println!("Out: {}", output);
                last_output_value = Some(output);
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

    return (program, last_output_value);
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
        assert_eq!(max_thruster_signal(&program), 43210);
    }

    #[test]
    fn test_max_thruster_example_two() {
        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(max_thruster_signal(&program), 54321);
    }

    #[test]
    fn test_max_thruster_with_feedback_example_one() {
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(max_thruster_signal_with_feedback(&program), 139629729);
    }
}
