use std::collections::HashMap;
use std::io::{stdin,stdout,Write};
use std::fmt;

fn main() {
    let program = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,902,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,37,1007,1102,24,1,1006,1102,26,1,1012,1101,528,0,1023,1102,256,1,1027,1102,466,1,1029,1102,1,629,1024,1101,0,620,1025,1101,0,0,1020,1102,1,30,1004,1101,39,0,1003,1102,36,1,1005,1102,531,1,1022,1102,32,1,1019,1101,0,27,1000,1101,0,28,1016,1101,1,0,1021,1101,23,0,1013,1102,1,25,1015,1102,1,21,1008,1102,1,22,1018,1102,1,34,1014,1102,475,1,1028,1101,33,0,1002,1101,0,35,1011,1102,1,20,1009,1102,38,1,1017,1101,259,0,1026,1101,31,0,1010,1101,0,29,1001,109,8,21102,40,1,10,1008,1018,40,63,1005,63,203,4,187,1105,1,207,1001,64,1,64,1002,64,2,64,109,7,21108,41,41,0,1005,1015,225,4,213,1106,0,229,1001,64,1,64,1002,64,2,64,109,1,1205,5,247,4,235,1001,64,1,64,1105,1,247,1002,64,2,64,109,20,2106,0,-9,1105,1,265,4,253,1001,64,1,64,1002,64,2,64,109,-38,1202,4,1,63,1008,63,33,63,1005,63,291,4,271,1001,64,1,64,1106,0,291,1002,64,2,64,109,6,2102,1,0,63,1008,63,29,63,1005,63,315,1001,64,1,64,1106,0,317,4,297,1002,64,2,64,109,10,21102,42,1,5,1008,1019,40,63,1005,63,341,1001,64,1,64,1105,1,343,4,323,1002,64,2,64,109,-13,2101,0,5,63,1008,63,24,63,1005,63,365,4,349,1105,1,369,1001,64,1,64,1002,64,2,64,109,7,1202,-6,1,63,1008,63,36,63,1005,63,389,1105,1,395,4,375,1001,64,1,64,1002,64,2,64,109,1,2107,31,-5,63,1005,63,411,1106,0,417,4,401,1001,64,1,64,1002,64,2,64,109,3,1206,8,431,4,423,1105,1,435,1001,64,1,64,1002,64,2,64,109,-8,2108,31,0,63,1005,63,451,1105,1,457,4,441,1001,64,1,64,1002,64,2,64,109,26,2106,0,-2,4,463,1001,64,1,64,1106,0,475,1002,64,2,64,109,-33,1207,6,38,63,1005,63,491,1106,0,497,4,481,1001,64,1,64,1002,64,2,64,109,3,2108,27,0,63,1005,63,515,4,503,1105,1,519,1001,64,1,64,1002,64,2,64,109,23,2105,1,0,1106,0,537,4,525,1001,64,1,64,1002,64,2,64,109,-30,1207,7,28,63,1005,63,559,4,543,1001,64,1,64,1106,0,559,1002,64,2,64,109,20,21101,43,0,0,1008,1013,43,63,1005,63,581,4,565,1105,1,585,1001,64,1,64,1002,64,2,64,109,-14,2102,1,1,63,1008,63,27,63,1005,63,611,4,591,1001,64,1,64,1105,1,611,1002,64,2,64,109,18,2105,1,7,4,617,1001,64,1,64,1106,0,629,1002,64,2,64,109,13,1206,-9,641,1105,1,647,4,635,1001,64,1,64,1002,64,2,64,109,-18,21107,44,45,-1,1005,1011,665,4,653,1105,1,669,1001,64,1,64,1002,64,2,64,109,-2,2107,28,-9,63,1005,63,687,4,675,1106,0,691,1001,64,1,64,1002,64,2,64,1205,10,701,1106,0,707,4,695,1001,64,1,64,1002,64,2,64,109,-6,1201,2,0,63,1008,63,21,63,1005,63,731,1001,64,1,64,1106,0,733,4,713,1002,64,2,64,109,-5,1208,7,23,63,1005,63,753,1001,64,1,64,1105,1,755,4,739,1002,64,2,64,109,16,1208,-8,37,63,1005,63,777,4,761,1001,64,1,64,1106,0,777,1002,64,2,64,109,3,21107,45,44,-8,1005,1010,797,1001,64,1,64,1105,1,799,4,783,1002,64,2,64,109,-8,1201,-5,0,63,1008,63,36,63,1005,63,821,4,805,1106,0,825,1001,64,1,64,1002,64,2,64,109,-9,2101,0,1,63,1008,63,31,63,1005,63,845,1105,1,851,4,831,1001,64,1,64,1002,64,2,64,109,6,21108,46,49,3,1005,1010,867,1106,0,873,4,857,1001,64,1,64,1002,64,2,64,109,5,21101,47,0,7,1008,1019,44,63,1005,63,897,1001,64,1,64,1106,0,899,4,879,4,64,99,21101,27,0,1,21102,913,1,0,1106,0,920,21201,1,30449,1,204,1,99,109,3,1207,-2,3,63,1005,63,962,21201,-2,-1,1,21101,940,0,0,1105,1,920,21202,1,1,-1,21201,-2,-3,1,21102,1,955,0,1106,0,920,22201,1,-1,-2,1105,1,966,22102,1,-2,-2,109,-3,2105,1,0];

    let (_, outputs) = run_intcode_with_output(program);

    if outputs.len() == 1 {
        println!("BOOST keycode: {}", outputs[0]);
    } else {
        println!("Something's not right. Outputs: {:?}", outputs);
    }
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

fn get_input_as_int() -> i64 {
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

fn run_intcode_with_output(program: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut outputs = Vec::new();
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
                let input = get_input_as_int();
                let destination = resolve_destination(&ec, &parameter_modes, 1);
                ec.set(destination, input);
            }
            Opcode::Print => {
                let operands = resolve_operands(&ec, &parameter_modes, 1);
                let output_value = operands[0];
                println!("Out: {}", output_value);
                outputs.push(output_value);
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

    return (ec.program, outputs);
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
