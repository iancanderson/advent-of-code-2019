use std::collections::HashMap;
use std::fmt;
use std::sync::mpsc;

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
    AdjustRelativeBase,
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
    Relative,
}

fn char_to_parameter_mode(c: char) -> ParameterMode {
    match c {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        '2' => ParameterMode::Relative,
        _ => panic!("invalid parameter mode: {}", c)
    }
}

// Opcode integer is now more complicated..
fn int_to_opcode(int: i64) -> Opcode {
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
    } else if int == 9 {
        Opcode::AdjustRelativeBase
    } else if int == 99 {
        Opcode::EndOfProgram
    } else {
        panic!("Unknown opcode")
    }
}

// Given the first value of an instruction, return
// (Opcode, Vec<ParameterMode>)
// The parameter mode vector represents the parameter modes
// of each parameter in the instruction.
fn parse_first_value(first_value: i64) -> (Opcode, Vec<ParameterMode>) {
    // Get lst two digits for the opcode
    // Convert to string, then take last two characters, then parse to int
    // Needs to work for both "2" and "1002"
    let mut as_string = first_value.to_string();

    if as_string.len() <= 2 {
        let opcode_string = as_string;
        let opcode_int: i64 = opcode_string.parse().unwrap();
        return (int_to_opcode(opcode_int), Vec::new());
    } else {
        let opcode_string = as_string.split_off(as_string.len() - 2);
        let opcode_int: i64 = opcode_string.parse().unwrap();

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
        Opcode::AdjustRelativeBase => 2,
        Opcode::EndOfProgram => 1,
    }
}

// Resolves to a location
fn resolve_destination(ec: &ExecutionContext, parameter_modes: &Vec<ParameterMode>, position_offset: usize) -> i64 {
    let parameter_mode = parameter_modes.get(position_offset - 1).unwrap_or(&ParameterMode::Position);

    return match parameter_mode {
        ParameterMode::Position => ec.program[ec.current_position + position_offset],
        ParameterMode::Immediate => panic!("Immediate destination doesn't make sense"),
        ParameterMode::Relative => {
            let relative_index = ec.program[ec.current_position + position_offset];
            return ec.relative_base + relative_index;
        }
    };
}

// Resolves to a value
fn resolve_operands(ec: &ExecutionContext, parameter_modes: &Vec<ParameterMode>, num_operands: usize) -> Vec<i64> {
    let debug = false;

    // Loop over operands
    let operands: Vec<i64> = (0..num_operands).map(|operand_offset| -> i64 {
        let index = ec.current_position + operand_offset + 1;
        let operand = ec.program[index];

        let parameter_mode = parameter_modes.get(operand_offset).unwrap_or(&ParameterMode::Position);

        let resolved_operand = match parameter_mode {
            ParameterMode::Position => ec.read_value(operand),
            ParameterMode::Immediate => operand,
            ParameterMode::Relative => ec.read_value(ec.relative_base as i64 + operand),
        };

        if debug {
            println!("raw operand: {}", operand);
            println!("resolved operand: {}", resolved_operand);
        }

        return resolved_operand;
    }).collect();

    return operands;
}

struct ExecutionContext {
    program: Vec<i64>,
    extended_memory: HashMap<i64, i64>,
    relative_base: i64,
    current_position: usize,
}

impl ExecutionContext {
    // Either reads from a program register, or from extended memory
    fn read_value(&self, index: i64) -> i64 {
        if index < 0 { panic!("Can't access negative memory address") }

        if self.program.len() > index as usize {
            return self.program[index as usize];
        } else {
            // Memory defaults to 0
            return *self.extended_memory.get(&index).unwrap_or(&0);
        }
    }

    // Either sets a program register or extended memory register
    fn set(&mut self, index: i64, value: i64) {
        if index < 0 { panic!("Can't access negative memory address") }

        if self.program.len() > index as usize {
            self.program[index as usize] = value;
        } else {
            self.extended_memory.insert(index, value);
        }
    }

    fn adjust_relative_base(&mut self, delta: i64) {
        self.relative_base += delta;
    }

    fn first_value_of_current_line(&self) -> i64 {
        return self.program[self.current_position];
    }
}

pub fn run_intcode_with_channels(program: Vec<i64>, input: mpsc::Receiver<i64>, output: mpsc::Sender<i64>) -> Vec<i64> {
    let debug = false;

    let mut ec = ExecutionContext {
        program,
        extended_memory: HashMap::new(),
        relative_base: 0,
        current_position: 0,
    };

    loop {
        let (current_opcode, parameter_modes) = parse_first_value(ec.first_value_of_current_line());

        if debug {
            println!("");
            println!("Running {} at position {}", current_opcode, ec.current_position);
            println!("Relative base is {}", ec.relative_base);
            println!("Parameter modes: {:?}", parameter_modes);
        }

        match current_opcode {
            Opcode::Add => {
                let operands = resolve_operands(&ec, &parameter_modes, 2);
                let destination = resolve_destination(&ec, &parameter_modes, 3);
                ec.set(destination, operands[0] + operands[1]);
            }
            Opcode::Multiply => {
                let operands = resolve_operands(&ec, &parameter_modes, 2);
                let destination = resolve_destination(&ec, &parameter_modes, 3);
                ec.set(destination, operands[0] * operands[1]);
            }
            Opcode::GetInput => {
                let input = input.recv().unwrap();
                let destination = resolve_destination(&ec, &parameter_modes, 1);
                ec.set(destination, input);
            }
            Opcode::Print => {
                let operands = resolve_operands(&ec, &parameter_modes, 1);
                let output_value = operands[0];
                // println!("Out: {}", output_value);
                output.send(output_value).unwrap();
            }
            Opcode::JumpIfTrue => {
                let operands = resolve_operands(&ec, &parameter_modes, 2);

                if operands[0] != 0 {
                    ec.current_position = operands[1] as usize;
                    continue;
                }
            }
            Opcode::JumpIfFalse => {
                let operands = resolve_operands(&ec, &parameter_modes, 2);

                if operands[0] == 0 {
                    ec.current_position = operands[1] as usize;
                    continue;
                }
            }
            Opcode::LessThan => {
                let operands = resolve_operands(&ec, &parameter_modes, 2);
                let destination = resolve_destination(&ec, &parameter_modes, 3);

                let value =
                    if operands[0] < operands[1] {
                        1
                    } else {
                        0
                    };

                ec.set(destination, value);
            }
            Opcode::Equals => {
                let operands = resolve_operands(&ec, &parameter_modes, 2);
                let destination = resolve_destination(&ec, &parameter_modes, 3);
                let value =
                    if operands[0] == operands[1] {
                        1
                    } else {
                        0
                    };

                ec.set(destination, value);
            }
            Opcode::AdjustRelativeBase => {
                let operands = resolve_operands(&ec, &parameter_modes, 1);
                ec.adjust_relative_base(operands[0]);
            }
            Opcode::EndOfProgram => {
                break;
            }
        };

        ec.current_position += num_values_in_instruction(current_opcode);
    }

    // Send -99 to signal that we're done
    output.send(-99).unwrap();

    return ec.program;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_intcode(program: Vec<i64>) -> Vec<i64> {
        return run_intcode_with_output(program).0;
    }

    #[test]
    fn test_relative_destination() {
        let program = vec![
            109,5, // Set relative base to 5
            20001,0,0,-5,99,
        ];
        let answer = run_intcode(program);
        assert_eq!(answer, vec![218,5,20001,0,0,-5,99]);
    }

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
    fn test_nine_example_one() {
        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let (_, outputs) = run_intcode_with_output(program.clone());
        assert_eq!(outputs, program);
    }

    #[test]
    fn test_nine_example_two() {
        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        let (_, outputs) = run_intcode_with_output(program);
        assert_eq!(outputs, [1219070632396864]);
    }

    #[test]
    fn test_nine_example_three() {
        let program = vec![104,1125899906842624,99];
        let (_, outputs) = run_intcode_with_output(program);
        assert_eq!(outputs, [1125899906842624]);
    }
}
