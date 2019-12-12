use std::fmt;
use std::thread;
use std::sync::mpsc;
use itertools::{Itertools, Permutations};

fn main() {
    let program = vec![3,8,1001,8,10,8,105,1,0,0,21,30,51,72,81,94,175,256,337,418,99999,3,9,101,5,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,1001,9,2,9,1002,9,5,9,4,9,99,3,9,1002,9,4,9,101,4,9,9,102,5,9,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,4,9,99,3,9,102,3,9,9,1001,9,4,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,99];

    println!("Max thruster signal: {}", max_thruster_signal_with_feedback(&program));
}

// Simple version of program
//
// Each amplifier needs to be able to receive input from previous amplifier,
// and produce output for the next amplifier.
//
// Proof of concept program:
//
// Initialize each amplifier
// Arrange amplifiers in a sequence
// When an amplifier receives an input value, it produces that value + 1 to its output
//
// In the main thread:
// Provide a starting value to the first amplifier
// Read from the output of the last amplifier to get the result

fn max_thruster_signal_with_feedback(program: &Vec<i32>) -> i32 {
    // Loop over all permutations of [0,1,2,3,4]
    // Find permutation with max thruster signal

    return vec![5,6,7,8,9].iter().permutations(5).map(|phase_setting_sequence| {
        let signal = thruster_signal_with_feedback(&phase_setting_sequence, program);
        println!("Sequence {:?} gives output signal: {}", phase_setting_sequence, signal);
        return signal;
    }).max().expect("There will definitely be a maximum");
}

fn thruster_signal_with_feedback(phase_setting_sequence: &Vec<&i32>, program: &Vec<i32>) -> i32 {
    // This channel goes into the first amplifier
    let (first_output, first_input) = mpsc::channel();
    let mut last_input: mpsc::Receiver<i32> = first_input;

    let mut child_threads = vec![];

    for phase_setting in phase_setting_sequence {
        // This channel goes out of the amplifier
        let (tx, rx) = mpsc::channel();
        let program_copy = program.clone();
        let phase_setting_value = **phase_setting;

        child_threads.push(thread::spawn(move || {
            let amp = Amplifier { output: tx, input: last_input };

            run_intcode_with_channel(program_copy, phase_setting_value, amp.input, amp.output);
        }));

        last_input = rx;
    }

    // Send 0 into the first amplifier
    first_output.send(0).unwrap();

    let received = last_input.recv().unwrap();
    println!("Received: {}", received);

    first_output.send(received).unwrap();

    for child_thread in child_threads {
        let _ = child_thread.join();
    }

    let received2 = last_input.recv().unwrap();
    println!("Received: {}", received2);

    return received2;

    // How to know when all threads halt?
    // I guess the thread should return when they get a 99 instruction
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

// Returns when it gets a 99 instruction
// Returns nothing, only outputs to the channel
fn run_intcode_with_channel(mut program: Vec<i32>, initial_input: i32, input: mpsc::Receiver<i32>, output: mpsc::Sender<i32>) {
    let mut current_position = 0;
    let mut has_consumed_initial_input = false;

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
                    if has_consumed_initial_input {
                        input.recv().unwrap()
                    } else {
                        has_consumed_initial_input = true;
                        initial_input
                    };

                println!("Resolved input: {}", resolved_input);

                let operand = program[current_position + 1];
                program[operand as usize] = resolved_input;
            }
            Opcode::Print => {
                let operand = program[current_position + 1];
                let output_value = program[operand as usize];

                println!("Out: {}", output_value);
                output.send(output_value).unwrap()
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
}

struct Amplifier {
    output: mpsc::Sender<i32>,
    input: mpsc::Receiver<i32>,
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_max_thruster_example_one() {
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(max_thruster_signal_with_feedback(&program), 139629729);
    }
}
